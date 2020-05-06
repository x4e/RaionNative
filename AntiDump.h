#pragma once
#include "Utils.hpp"
#include <jni.h>
#include <jvmti.h>
#include <cstdint>

#ifndef RAIONNATIVE_ANTIDUMP_H
#define RAIONNATIVE_ANTIDUMP_H



typedef struct {
	const char* typeName;            // The type name containing the given field (example: "Klass")
	const char* fieldName;           // The field name within the type           (example: "_name")
	const char* typeString;          // Quoted name of the type of this field (example: "Symbol*";
	// parsed in Java to ensure type correctness
	int32_t  isStatic;               // Indicates whether following field is an offset or an address
	uint64_t offset;                 // Offset of field within structure; only used for nonstatic fields
	void* address;                   // Address of field; only used for static fields
	// ("offset" can not be reused because of apparent SparcWorks compiler bug
	// in generation of initializer data)
} VMStructEntry;

typedef struct {
	const char* typeName;            // Type name (example: "Method")
	const char* superclassName;      // Superclass name, or null if none (example: "oopDesc")
	int32_t isOopType;               // Does this type represent an oop typedef? (i.e., "Method*" or
	// "Klass*", but NOT "Method")
	int32_t isIntegerType;           // Does this type represent an integer type (of arbitrary size)?
	int32_t isUnsigned;              // If so, is it unsigned?
	uint64_t size;                   // Size, in bytes, of the type
} VMTypeEntry;

typedef struct {
	const char* name;                // Name of constant (example: "_thread_in_native")
	int32_t value;                   // Value of constant
} VMIntConstantEntry;

typedef struct {
	const char* name;                // Name of constant (example: "_thread_in_native")
	uint64_t value;                  // Value of constant
} VMLongConstantEntry;




void fuckAttachThread(JNIEnv *env, jvmtiEnv* jvmti);
void fuckStructs(JNIEnv *env, jvmtiEnv* jvmti);

#endif //RAIONNATIVE_ANTIDUMP_H
