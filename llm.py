import argparse
import copy
import dataclasses
import json
import logging
import os
import time
from dataclasses import dataclass
from typing import Optional, Any

import boto3
import botocore
import dacite
from tenacity import retry, retry_if_exception_type, wait_random_exponential, stop_after_delay


@dataclass
class Message:
    Role: str
    Content: str


@dataclass
class Conversation:
    # there is always only one system prompt, and it goes at the very beginning,
    # so have a separate field for it
    System: str
    Messages: list[Message]

    def claude_messages(self):
        l = []
        for m in self.Messages:
            l.append({"role": m.Role, "content": m.Content})
        return l

    @staticmethod
    def new_convo(system: str, prompt: str):
        return Conversation(System=system, Messages=[Message(Role="user", Content=prompt)])

    def add_response(self, prompt: str):
        self.Messages.append(Message(Role="user", Content=prompt))

    # I define the str() method this way so that the summary of a Conversation object in a debugger shows the
    # content of the llm's response
    def __str__(self):
        return self.Messages[-1].Content

    def to_string(self):
        return f"{self.System}\n" + "\n".join([m.Content for m in self.Messages])

    def last_message(self) -> str:
        return self.Messages[-1].Content


class QueryError(Exception):
    """
    A wrapper around all sorts of errors thrown by LLMs
    """

    pass


@dataclass
class LLMQueryInput:
    Conversation: Conversation
    Params: dict[str, Any]
    StopSequences: list[str]

    @staticmethod
    def construct_llm_query_input(convo: Conversation, temperature: float, stop_sequences: list[str] = []):
        return LLMQueryInput(
            Conversation=convo,
            Params={"temperature": temperature},
            StopSequences=stop_sequences
        )


@dataclass
class LLMQueryOutput:
    Conversation: Conversation
    Meta: dict[str, Any]

    def __str__(self):
        return self.Conversation.Messages[-1].Content


class LLMQuery:
    def query(self, i: LLMQueryInput) -> LLMQueryOutput:
        raise Exception("Implement me")


class Claude(LLMQuery):
    """
    An object for querying Anthropic Claude in bedrock.
    """

    def __init__(self,
                 logger: logging.Logger,
                 model_id: str = "anthropic.claude-3-sonnet-20240229-v1:0",
                 profile_name: Optional[str] = None,
                 region: Optional[str] = None):
        """
        :param logger: see LLM base class
        :param model_id: the bedrock model id. Should be one of the Claude models
        :param profile_name: the name of the aws profile to use in ~/.aws/config
        """
        config = botocore.config.Config(
            read_timeout=900, connect_timeout=900, retries={"max_attempts": 0}
        )
        if profile_name:
            sess = boto3.Session(profile_name=profile_name)
        else:
            sess = boto3.Session()
        self.profile_name = profile_name
        self.bedrock = sess.client(service_name="bedrock-runtime", config=config, region_name=region)
        self.model_id = model_id
        self.logger = logger

    def query(self, i: LLMQueryInput) -> LLMQueryOutput:
        inp = {
            "max_tokens": 8192,
            "messages": i.Conversation.claude_messages(),
            "system": i.Conversation.System,
            "anthropic_version": "bedrock-2023-05-31",
            "stop_sequences": i.StopSequences
        }
        inp.update(i.Params)
        body = json.dumps(inp)

        start = time.time()
        try:
            response = self.bedrock.invoke_model(body=body, modelId=self.model_id)
        except Exception as e:
            print(
                f"Exception when querying bedrock with profile {self.profile_name} in region {self.bedrock.meta.region_name}: {e}")
            raise QueryError(e)
        elapsed = time.time() - start

        # Response body structure:
        # {
        #     "id": string,
        #     "model": string,
        #     "type" : "message",
        #     "role" : "assistant",
        #     "content": [
        #         {
        #             "type": "text",
        #             "text": string
        #         }
        #     ],
        #     "stop_reason": string,
        #     "stop_sequence": string,
        #     "usage": {
        #         "input_tokens": integer,
        #         "output_tokens": integer
        #     }
        #
        # }
        response_body = json.loads(response.get("body").read())
        response = response_body["content"][0]["text"]
        if response_body["stop_reason"] == "stop_sequence":
            response += response_body["stop_sequence"]
        ret = copy.deepcopy(i.Conversation)
        ret.Messages.append(Message("assistant", response))
        meta = response_body["usage"]
        meta["time"] = elapsed
        return LLMQueryOutput(ret, meta)


class LLMRetry(LLMQuery):
    def __init__(self, llm_query: LLMQuery):
        self.llm_query = llm_query

    @retry(
        reraise=True,
        retry=retry_if_exception_type(QueryError),
        wait=wait_random_exponential(multiplier=1, max=120),
        stop=stop_after_delay(900),
    )
    def query(self, i: LLMQueryInput) -> LLMQueryOutput:
        return self.llm_query.query(i)


@dataclass
class ReplayableSaved:
    Input: LLMQueryInput
    Output: LLMQueryOutput


class LLMReplayable(LLMQuery):
    FILE_PREFIX = "prompt_"

    def __init__(self, logger: logging.Logger, llm_query: LLMQuery, replay_directory: str, replay: bool,
                 strict_replay: bool):
        """
        :param replay: whether to look in replay_directory for prompts to replay. If the replay replay_directory does
        not exist, behaves as if replay=False. If replay_directory exists, but a saved response does not exist for the
        query input, performs a query and saves the result. If a saved response exists but the input in the saved
        response does not match the runtime query input, it will perform the query and overwrite the saved response.
        :param strict_replay: fail if a saved response with a matching input does not exist for the call to query()
        """
        self.llm_query = llm_query
        self.replay_directory = replay_directory
        self.replay = replay
        self.strict_replay = strict_replay
        self.query_counter = 0
        self.logger = logger

        if self.replay and self.strict_replay and not os.path.exists(self.replay_directory):
            raise Exception(f"Replay directory does not exist {self.replay_directory}")
        else:
            if self.strict_replay and not self.replay:
                raise Exception(f"replay=False but strict_replay=True")
            os.makedirs(self.replay_directory, exist_ok=True)

    def _write_query(self, i: LLMQueryInput, o: LLMQueryOutput):
        path = f"{self.replay_directory}/{self.FILE_PREFIX}{self.query_counter}"
        if self.strict_replay:
            # defensive programming
            raise Exception("strict_replay=True but saving a response?")
        with open(path, "w+") as fd:
            fd.write(json.dumps(dataclasses.asdict(ReplayableSaved(i, o))))

    def _read_query(self, i: LLMQueryInput) -> Optional[LLMQueryOutput]:
        path = f"{self.replay_directory}/{self.FILE_PREFIX}{self.query_counter}"
        if os.path.exists(path):
            with open(path) as fd:
                json_string = fd.read()
                if not json_string:
                    # sometimes we end up with empty files
                    return None
                saved = dacite.from_dict(ReplayableSaved, json.loads(json_string))
            if saved.Input == i:
                return saved.Output
            else:
                return None
        else:
            return None

    def _do_query(self, i: LLMQueryInput) -> LLMQueryOutput:
        if self.replay:
            o = self._read_query(i)
            if o is not None:
                print("Using cache")
                return o
            elif self.strict_replay:
                raise Exception("Query output not found and self.strict_replay=True")
        o = self.llm_query.query(i)
        self._write_query(i, o)
        return o

    def query(self, i: LLMQueryInput) -> LLMQueryOutput:
        o = self._do_query(i)
        self.query_counter += 1
        return o


def make_cached_llm(log_directory: str, replay: bool, profile: str) -> LLMQuery:
    return LLMReplayable(
        logging.getLogger("test"),
        LLMRetry(
            Claude(
                logging.getLogger("test"),
                "anthropic.claude-3-5-sonnet-20240620-v1:0",
                profile,
            )
        ),
        log_directory,
        replay,
        replay
    )


USAGE = """
An LLM class that provides retry on failure and replaying logged responses.
"""


def parse_args():
    parser = argparse.ArgumentParser(description="Generate a runbook from natural language")
    parser.add_argument("--prompt", type=str, required=True,
                        help="The prompt for the LLM")
    parser.add_argument("--log-directory", required=True, type=str, default="response_log",
                        help="Directory to log responses to")
    parser.add_argument("--replay", action="store_true",
                        help="Replay the LLM responses in the --log-directory directory. "
                             "Throw an exception if the runtime LLM inputs do not match logged LLM inputs.")
    parser.add_argument("--temperature", type=float, default=0.2, help="The temperature hyperparameter")

    args = parser.parse_args()
    return args


def main():
    args = parse_args()
    llm = make_cached_llm(args.log_directory, args.replay)
    response = llm.query(
        LLMQueryInput.construct_llm_query_input(
            Conversation.new_convo(
                "You are a helpful person",
                args.prompt
            ),
            args.temperature
        )
    )

    convo = response.Conversation
    print("######### Response recieved: ")
    print(convo.last_message())

    print("\n########Asking for a second response\n")
    convo.add_response("Could you give me another answer?")

    response = llm.query(
        LLMQueryInput.construct_llm_query_input(
            convo,
            args.temperature
        )
    )

    print("######### Response recieved: ")
    print(response.Conversation.last_message())

if __name__ == "__main__":
    main()
