#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <assert.h>

char *URL_SCHEMES[] = {
  // official IANA registered schemes
  "aaa",  "javascript", "jdbc", "doi"
};

static char *
strff (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr++;
  }

  return strdup(ptr);
}

static char *
strrwd (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr--;
  }

  return strdup(ptr);
}

bool
url_is_protocol (char *str) {
  int count = sizeof(URL_SCHEMES) / sizeof(URL_SCHEMES[0]);

  for (int i = 0; i < count; ++i) {
    if (0 == strcmp(URL_SCHEMES[i], str)) {
      return true;
    }
  }

  return false;
}

bool
url_is_ssh (char *str) {
  str = strdup(str);
  if (0 == strcmp(str, "ssh") ||
      0 == strcmp(str, "git")) {
    free(str);
    return true;

  }

  return false;
}
#define URL_PROTOCOL_MAX_LENGTH 16

static char *
get_part (char *url, const char *format, int l) {
  bool has = false;
  char *tmp = malloc(sizeof(char));
  char *tmp_url = strdup(url);
  char *fmt_url = strdup(url);
  char *ret = malloc(sizeof(char));

  if (!tmp || !tmp_url || !fmt_url || !ret)
    return NULL;

  strcpy(tmp, "");
  strcpy(fmt_url, "");

  // move pointer exactly the amount
  // of characters in the `prototcol` char
  // plus 3 characters that represent the `://`
  // part of the url
  fmt_url = strff(fmt_url, l);

  sscanf(fmt_url, format, tmp);

  if (0 != strcmp(tmp, tmp_url)) {
    has = true;
    ret = strdup(tmp);
  }

  // descrement pointer to original
  // position so it can be free'd
  fmt_url = strrwd(fmt_url, l);

  free(tmp);
  free(tmp_url);
  free(fmt_url);

  return has? ret : NULL;
}

char * url_get_protocol (char *url) {
  char *protocol = malloc(URL_PROTOCOL_MAX_LENGTH * sizeof(char));
  if (!protocol) return NULL;
  sscanf(url, "%[^://]", protocol);
  if (url_is_protocol(protocol)) return protocol;
  return NULL;
}

char * url_get_auth (char *url) {
  char *protocol = url_get_protocol(url);
  if (!protocol) return NULL;
  int l = (int) strlen(protocol) + 3;
  return get_part(url, "%[^@]", l);
}

char * url_get_hostname (char *url) {
  int l = 3;
  char *protocol = url_get_protocol(url);
  char *tmp_protocol = strdup(protocol);
  char *auth = url_get_auth(url);

  if (!protocol) return NULL;
  if (auth) l += strlen(auth) + 1; // add one @ symbol
  if (auth) free(auth);

  l += (int) strlen(protocol);

  free(protocol);

  char * hostname = url_is_ssh(tmp_protocol)
           ? get_part(url, "%[^:]", l)
           : get_part(url, "%[^/]", l);
  free(tmp_protocol);
  return hostname;
}

char *
url_get_path (char *url) {
  int l = 3;
  char *tmp_path;
  char *protocol = url_get_protocol(url);
  char *auth = url_get_auth(url);
  char *hostname = url_get_hostname(url);


  if (!protocol || !hostname)
    return NULL;

  bool is_ssh = url_is_ssh(protocol);

  l += (int) strlen(protocol) + (int) strlen(hostname);

  if (auth) l+= (int) strlen(auth) +1; // @ symbol

  tmp_path = (is_ssh)
              ? get_part(url, ":%s", l)
              : get_part(url, "/%s", l);

  char *fmt = (is_ssh)? "%s" : "/%s";
  char *path = malloc(strlen(tmp_path) * sizeof(char));
  sprintf(path, fmt, tmp_path);

  if (auth) free(auth);
  free(protocol);
  free(hostname);
  free(tmp_path);

  return path;

}

int
main (void) {
  char *url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
  assert(0 == strcmp("/p/a/t/h?query=string#hash", url_get_path(url)));
}
