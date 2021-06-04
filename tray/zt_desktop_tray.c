#include <stdio.h>
#include <string.h>

#if defined (_WIN32) || defined (_WIN64)
#define TRAY_WINAPI 1
#elif defined (__linux__) || defined (linux) || defined (__linux)
#define TRAY_APPINDICATOR 1
#elif defined (__APPLE__) || defined (__MACH__)
#define TRAY_APPKIT 1
#endif

#include "tray.h"

#ifdef __APPLE__
#include <pthread.h>
#include <pthread/qos.h>
#include <sys/qos.h>
extern void c_set_this_thread_to_background_priority()
{
	pthread_set_qos_class_self_np(QOS_CLASS_BACKGROUND, 0);
}
extern void c_set_this_thread_to_foreground_priority()
{
	pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, 0);
}
#endif
