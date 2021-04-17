#define _AMD64_
#pragma once
#include "AntiDump.h"
#include "Libloaderapi.h"

void fuckAttachThread(JNIEnv *env, jvmtiEnv* jvmti) {
}
void fuckStructs(JNIEnv *env, jvmtiEnv* jvmti) {
	auto java_dll = GetModuleHandleA("java.dll");
	
	if (!java_dll) {
		println("Unable to find java.dll");
		g_vm->DestroyJavaVM();
	}
	
	auto structs = GetProcAddress(java_dll, "gHotSpotVMStructs");
	
	println("Structs '%d'", structs);
	
	if (!structs) {
		println("Unable to find structs");
		g_vm->DestroyJavaVM();
	}
	
	auto cStructs = (VMStructEntry*)structs;
	
	if (!cStructs) {
		println("Unable to cast structs");
		g_vm->DestroyJavaVM();
	}
	
	__int64 addr = -1;
	{
		auto clazz = env->FindClass("java/lang/ClassLoader");
		assertNoException(env);
		println("1");
		
		auto sysMethod = getMethod(env, clazz, "getSystemClassLoader", "()Ljava/lang/ClassLoader;", true);
		assertNoException(env);
		assert(sysMethod);
		println("2");
		auto sys = env->CallStaticObjectMethod(clazz, sysMethod);
		assert(sys);
		println("3");
		
		auto method = getMethod(env, clazz, "findNative", "(Ljava/lang/ClassLoader;Ljava/lang/String;)J", true);
		assertNoException(env);
		assert(method);
		println("4");
		
		auto str = env->NewStringUTF("gHotSpotVMStructs");
		assert(str);
		
		addr = env->CallStaticLongMethod(clazz, method, sys, str);
		assertNoException(env);
		assert(addr);
		println("5");
	}
	
	if (addr <= 0) {
		println("Couldnt find gHotSpotVMStructs");
		g_vm->DestroyJavaVM();
	}
	
	auto gHotSpotVMStructs = (VMStructEntry*)addr;
	assert(gHotSpotVMStructs);
}
