#pragma once
#define _AMD64_
#include <cstring>
#include "library.h"
#include "AntiDump.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Winvalid-token-paste"
#pragma ide diagnostic ignored "OCUnusedGlobalDeclarationInspection"

static jobject g_byteMap;

static void JNICALL class_file_load_hook_handler(
		jvmtiEnv* jvmti,
		JNIEnv* env,
		jclass class_being_redefined,
		jobject loader,
		const char* name,
		jobject protection_domain,
		jint class_data_len,
		const unsigned char* class_data,
		jint* new_class_data_len,
		unsigned char** new_class_data
) {
	if (name == nullptr) {
		return;
	}
	
	auto removeMethod = getMethod(env, "java/util/Map", "get", "(Ljava/lang/Object;)Ljava/lang/Object;");
	assertNoException(env);
	
	assert(g_byteMap);
	assert(removeMethod);
	auto jName = env->NewStringUTF(name);
	assertNoException(env);
	
	assert(jName);
	
	auto outObj = env->CallObjectMethod(g_byteMap, removeMethod, jName);
	assertNoException(env);
	
	if (outObj == nullptr) {
		return;
	} else {
		auto bArr = (jbyteArray)outObj;
		assert(bArr);
		auto bElements = env->GetByteArrayElements(bArr, JNI_FALSE);
		assertNoException(env);
		
		assert(bElements);
		
		auto len = env->GetArrayLength(bArr);
		assertNoException(env);
		
		assert(len);
		
		unsigned char* newData = nullptr;
		assert(jvmti->Allocate(len, &newData) == JVMTI_ERROR_NONE);
		assertNoException(env);
		assert(newData);
		
		for (int i = 0; i < len; i++) {
			newData[i] = bElements[i];
		}
		
		*new_class_data_len = len;
		*new_class_data = newData;
		assertNoException(env);
		return;
	}
}

static void JNICALL on_exception_callback(jvmtiEnv *jvmti_env,
		 JNIEnv* jni_env,
		 jthread thread,
		 jmethodID method,
		 jlocation location,
		 jobject exception,
		 jmethodID catch_method,
		 jlocation catch_location) {
	auto printMethod = getMethod(jni_env, "java/lang/Throwable", "printStacktrace", "()V");
	jni_env->CallVoidMethod(exception, printMethod);
}

void JNICALL
nativeMethodCallback(jvmtiEnv *jvmti_env,
                 JNIEnv* jni_env,
                 jthread thread,
                 jmethodID method,
                 void* address,
                 void** new_address_ptr) {
	//char* nameBuff = new char[100];
	//char* sigBuff = new char[100];
	//char* genericBuff = new char[100];
	//auto name = jvmti_env->GetMethodName(method, &nameBuff, &sigBuff, &genericBuff);
	
	//println("%s.%s.%s", nameBuff, sigBuff, genericBuff);
}

JNIEXPORT void JNICALL Java_me_cookiedragon234_falcon_NativeAccessor_b
		(JNIEnv *env, jobject jobj, jstring jstr, jobject byteMap, jobject classLoader, jobject redefinitions) {
	auto str = env->GetStringUTFChars(jstr, JNI_FALSE);
	println("Hello '%s'", str);
	
	
	g_byteMap = env->NewGlobalRef(byteMap);
	
	jvmtiEnv* jvmti;
	g_vm->GetEnv((void**)&jvmti, JVMTI_VERSION_1_2);
	assert(jvmti);
	
	// Capabilities
	{
		jvmtiCapabilities capabilities = {};
		ZERO(capabilities);
		capabilities.can_retransform_classes = 1;
		capabilities.can_generate_native_method_bind_events = 1;
		auto result = jvmti->AddCapabilities(&capabilities);
		if (result != JVMTI_ERROR_NONE) {
			println("Error adding capabilities %d", result);
			g_vm->DestroyJavaVM();
		}
	}
	
	// Callback
	{
		jvmtiEventCallbacks callbacks;
		ZERO(callbacks);
		callbacks.ClassFileLoadHook = &class_file_load_hook_handler;
		callbacks.NativeMethodBind = &nativeMethodCallback;
		assert(jvmti->SetEventCallbacks(&callbacks, sizeof(callbacks)) == JVMTI_ERROR_NONE);
	}
	
	// Enable event
	{
		// nullptr implies current thread
		assert(jvmti->SetEventNotificationMode(JVMTI_ENABLE, JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, nullptr) == JVMTI_ERROR_NONE);
		assert(jvmti->SetEventNotificationMode(JVMTI_ENABLE, JVMTI_EVENT_NATIVE_METHOD_BIND, nullptr) == JVMTI_ERROR_NONE);
	}
	
	println("Finished native callbacks");
	
	auto mapSizeMethod = getMethod(env, "java/util/Map", "size", "()I");
	auto entrySetMethod = getMethod(env, "java/util/Map", "entrySet", "()Ljava/util/Set;");
	auto iteratorMethod = getMethod(env, "java/util/Set", "iterator", "()Ljava/util/Iterator;");
	auto hasNextMethod = getMethod(env, "java/util/Iterator", "hasNext", "()Z");
	auto nextMethod = getMethod(env, "java/util/Iterator", "next", "()Ljava/lang/Object;");
	auto keyMethod = getMethod(env, "java/util/Map$Entry", "getKey", "()Ljava/lang/Object;");
	auto valueMethod = getMethod(env, "java/util/Map$Entry", "getValue", "()Ljava/lang/Object;");
	assert(mapSizeMethod);
	assert(entrySetMethod);
	assert(iteratorMethod);
	assert(hasNextMethod);
	assert(nextMethod);
	assert(keyMethod);
	assert(valueMethod);
	
	/*jint mapSize = env->CallIntMethod(byteMap, mapSizeMethod);
	
	jobject entrySet = env->CallObjectMethod(byteMap, entrySetMethod);
	
	jobject iterator = env->CallObjectMethod(entrySet, iteratorMethod4);
	
	
	while (env->CallBooleanMethod(iterator, hasNextMethod)) {
		auto next = env->CallObjectMethod(iterator, nextMethod);
		assert(next);
		
		auto key = (jstring)env->CallObjectMethod(next, keyMethod);
		auto value = (jbyteArray)env->CallObjectMethod(next, valueMethod);
		assert(key);
		assert(value);
		
		auto name = env->GetStringUTFChars(key, JNI_FALSE);
		auto bytes = env->GetByteArrayElements(value, JNI_FALSE);
		auto byteSize = env->GetArrayLength(value);
		assert(name);
		assert(bytes);
		
		auto clazz = env->DefineClass(name, classLoader, bytes, byteSize);
		//assertNoException(env);
		if (env->ExceptionCheck()) {
			env->ExceptionDescribe();
			env->ExceptionClear();
		}
		println("Defined %s", name);
	}*/
	int numRedefines = env->CallIntMethod(redefinitions, mapSizeMethod);
	auto redefineArr = new jclass[numRedefines];
	{
		auto entrySet = env->CallObjectMethod(redefinitions, entrySetMethod);
		assert(entrySet);
		auto iterator = env->CallObjectMethod(entrySet, iteratorMethod);
		assert(iterator);
		
		int i = 0;
		while (env->CallBooleanMethod(iterator, hasNextMethod)) {
			auto nextEntry = env->CallObjectMethod(iterator, nextMethod);
			assert(nextEntry);
			
			auto klazz = (jclass)env->CallObjectMethod(nextEntry, keyMethod);
			assert(klazz);
			auto bytes = (jbyteArray)env->CallObjectMethod(nextEntry, valueMethod);
			assert(bytes);
			
			//auto bLength = env->GetArrayLength(bytes);
			//assert(bLength);
			//auto bElements = env->GetByteArrayElements(bytes, JNI_FALSE);
			//assert(bElements);
			
			auto toStringMethod = getMethod(env, "java/lang/Object", "toString", "()Ljava/lang/String;");
			assert(toStringMethod);
			auto strName = (jstring)env->CallObjectMethod(klazz, toStringMethod);
			assert(strName);
			
			auto str = env->GetStringUTFChars(strName, JNI_FALSE);
			assert(str);
			
			redefineArr[i] = klazz;
			i += 1;
		}
	}
	jvmti->RetransformClasses(numRedefines, redefineArr);
	
	fuckAttachThread(env, jvmti);
	fuckStructs(env, jvmti);
}

JNIEXPORT jint JNI_OnLoad
		(JavaVM *env, void *reserved) {
	g_vm = env;
	println("Raion Native Library Loaded");
	return JNI_VERSION_1_8;
}


#pragma clang diagnostic pop
