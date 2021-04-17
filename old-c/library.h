#pragma once
#include <jni.h>
#include <jvmti.h>
#include "Utils.hpp"
/* Header for class me_cookiedragon234_falcon_NativeAccessor */

#ifndef _Included_me_cookiedragon234_falcon_NativeAccessor
#define _Included_me_cookiedragon234_falcon_NativeAccessor
#ifdef __cplusplus
extern "C" {
#endif

/*
 * Class:     me_cookiedragon234_falcon_NativeAccessor
 * Method:    b
 * Signature: (Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_me_cookiedragon234_falcon_NativeAccessor_b
		(JNIEnv *, jobject, jstring, jobject, jobject, jobject);

JNIEXPORT jint JNI_OnLoad_RaionNative(JavaVM *, void *);

#ifdef __cplusplus
}
#endif
#endif
