#pragma once
#define _AMD64_
#include <cstring>
#include <cassert>
#include <jni.h>

#ifndef RAIONNATIVE_UTILS_HPP
#define RAIONNATIVE_UTILS_HPP

static JavaVM * g_vm;

#define ZERO(X) memset(&X, 0, sizeof(X))
#define print(...) printf(##__VA_ARGS__); fflush(stdout)
#define println(...) printf(##__VA_ARGS__); fflush(stdout); printf("\n"); fflush(stdout)
#define __int64 long long
#define assertNoException(env) if (env->ExceptionCheck()) {env->ExceptionDescribe();}

static jfieldID getField(JNIEnv *env, const jclass clazz, const char *field, const char *sig, const bool isStatic = false) {
	if (isStatic) {
		auto id = env->GetStaticFieldID(clazz, field, sig);
		return id;
	} else {
		auto id = env->GetFieldID(clazz, field, sig);
		return id;
	}
}

static jfieldID getField(JNIEnv *env, const char *clazz, const char *field, const char *sig, const bool isStatic = false) {
	return getField(env, env->FindClass(clazz), field, sig, isStatic);
}

static jmethodID getMethod(JNIEnv *env, const jclass clazz, const char *method, const char *sig, const bool isStatic = false) {
	if (isStatic) {
		auto id = env->GetStaticMethodID(clazz, method, sig);
		return id;
	} else {
		auto id = env->GetMethodID(clazz, method, sig);
		return id;
	}
}

static jmethodID getMethod(JNIEnv *env, const char *clazz, const char *method, const char *sig, const bool isStatic = false) {
	return getMethod(env, env->FindClass(clazz), method, sig, isStatic);
}


#endif //RAIONNATIVE_UTILS_HPP
