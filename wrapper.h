#define _XOPEN_SOURCE 700
#include <sys/fcntl.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <sys/inotify.h>
#include <sys/epoll.h>
#include <sys/syslog.h>
#include <dlfcn.h>
#include <ftw.h>
#include <pthread.h>
