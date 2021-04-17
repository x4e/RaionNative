#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use jni::sys::{jint, jobject, JNINativeInterface_, jlong, jmethodID, jclass, jboolean, JNIEnv, jfieldID, jvalue, jfloat, jdouble, jchar};
use crate::bitfield::__BindgenBitfieldUnit;

pub const JVMTI_VERSION_1: _bindgen_ty_1 = 805371904;
pub const JVMTI_VERSION_1_0: _bindgen_ty_1 = 805371904;
pub const JVMTI_VERSION_1_1: _bindgen_ty_1 = 805372160;
pub const JVMTI_VERSION_1_2: _bindgen_ty_1 = 805372416;
pub const JVMTI_VERSION: _bindgen_ty_1 = 805372417;

pub type _bindgen_ty_1 = u32;
pub type jvmtiEnv = *const jvmtiInterface_1_;
pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jrawMonitorID {
	_unused: [u8; 0],
}

pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = JNINativeInterface_;

pub const JVMTI_THREAD_STATE_ALIVE: _bindgen_ty_2 = 1;
pub const JVMTI_THREAD_STATE_TERMINATED: _bindgen_ty_2 = 2;
pub const JVMTI_THREAD_STATE_RUNNABLE: _bindgen_ty_2 = 4;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: _bindgen_ty_2 = 1024;
pub const JVMTI_THREAD_STATE_WAITING: _bindgen_ty_2 = 128;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: _bindgen_ty_2 = 16;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: _bindgen_ty_2 = 32;
pub const JVMTI_THREAD_STATE_SLEEPING: _bindgen_ty_2 = 64;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: _bindgen_ty_2 = 256;
pub const JVMTI_THREAD_STATE_PARKED: _bindgen_ty_2 = 512;
pub const JVMTI_THREAD_STATE_SUSPENDED: _bindgen_ty_2 = 1048576;
pub const JVMTI_THREAD_STATE_INTERRUPTED: _bindgen_ty_2 = 2097152;
pub const JVMTI_THREAD_STATE_IN_NATIVE: _bindgen_ty_2 = 4194304;
pub const JVMTI_THREAD_STATE_VENDOR_1: _bindgen_ty_2 = 268435456;
pub const JVMTI_THREAD_STATE_VENDOR_2: _bindgen_ty_2 = 536870912;
pub const JVMTI_THREAD_STATE_VENDOR_3: _bindgen_ty_2 = 1073741824;

pub type _bindgen_ty_2 = u32;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: _bindgen_ty_3 = 1207;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: _bindgen_ty_3 = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: _bindgen_ty_3 = 2;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: _bindgen_ty_3 = 5;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: _bindgen_ty_3 = 1025;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: _bindgen_ty_3 = 145;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: _bindgen_ty_3 = 161;

pub type _bindgen_ty_3 = u32;

pub const JVMTI_THREAD_MIN_PRIORITY: _bindgen_ty_4 = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: _bindgen_ty_4 = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: _bindgen_ty_4 = 10;

pub type _bindgen_ty_4 = u32;

pub const JVMTI_HEAP_FILTER_TAGGED: _bindgen_ty_5 = 4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: _bindgen_ty_5 = 8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: _bindgen_ty_5 = 16;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: _bindgen_ty_5 = 32;

pub type _bindgen_ty_5 = u32;

pub const JVMTI_VISIT_OBJECTS: _bindgen_ty_6 = 256;
pub const JVMTI_VISIT_ABORT: _bindgen_ty_6 = 32768;

pub type _bindgen_ty_6 = u32;

pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_CLASS: jvmtiHeapReferenceKind = 1;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_FIELD: jvmtiHeapReferenceKind = 2;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT: jvmtiHeapReferenceKind = 3;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_CLASS_LOADER: jvmtiHeapReferenceKind = 4;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_SIGNERS: jvmtiHeapReferenceKind = 5;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN: jvmtiHeapReferenceKind = 6;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_INTERFACE: jvmtiHeapReferenceKind = 7;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_STATIC_FIELD: jvmtiHeapReferenceKind = 8;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_CONSTANT_POOL: jvmtiHeapReferenceKind = 9;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_SUPERCLASS: jvmtiHeapReferenceKind = 10;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_JNI_GLOBAL: jvmtiHeapReferenceKind = 21;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_SYSTEM_CLASS: jvmtiHeapReferenceKind = 22;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_MONITOR: jvmtiHeapReferenceKind = 23;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_STACK_LOCAL: jvmtiHeapReferenceKind = 24;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_JNI_LOCAL: jvmtiHeapReferenceKind = 25;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_THREAD: jvmtiHeapReferenceKind = 26;
pub const jvmtiHeapReferenceKind_JVMTI_HEAP_REFERENCE_OTHER: jvmtiHeapReferenceKind = 27;

pub type jvmtiHeapReferenceKind = u32;

pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_BOOLEAN: jvmtiPrimitiveType = 90;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_BYTE: jvmtiPrimitiveType = 66;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_CHAR: jvmtiPrimitiveType = 67;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_SHORT: jvmtiPrimitiveType = 83;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_INT: jvmtiPrimitiveType = 73;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_LONG: jvmtiPrimitiveType = 74;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_FLOAT: jvmtiPrimitiveType = 70;
pub const jvmtiPrimitiveType_JVMTI_PRIMITIVE_TYPE_DOUBLE: jvmtiPrimitiveType = 68;

pub type jvmtiPrimitiveType = u32;

pub const jvmtiHeapObjectFilter_JVMTI_HEAP_OBJECT_TAGGED: jvmtiHeapObjectFilter = 1;
pub const jvmtiHeapObjectFilter_JVMTI_HEAP_OBJECT_UNTAGGED: jvmtiHeapObjectFilter = 2;
pub const jvmtiHeapObjectFilter_JVMTI_HEAP_OBJECT_EITHER: jvmtiHeapObjectFilter = 3;

pub type jvmtiHeapObjectFilter = u32;

pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_JNI_GLOBAL: jvmtiHeapRootKind = 1;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_SYSTEM_CLASS: jvmtiHeapRootKind = 2;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_MONITOR: jvmtiHeapRootKind = 3;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_STACK_LOCAL: jvmtiHeapRootKind = 4;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_JNI_LOCAL: jvmtiHeapRootKind = 5;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_THREAD: jvmtiHeapRootKind = 6;
pub const jvmtiHeapRootKind_JVMTI_HEAP_ROOT_OTHER: jvmtiHeapRootKind = 7;

pub type jvmtiHeapRootKind = u32;

pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_CLASS: jvmtiObjectReferenceKind = 1;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_FIELD: jvmtiObjectReferenceKind = 2;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_ARRAY_ELEMENT: jvmtiObjectReferenceKind = 3;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_CLASS_LOADER: jvmtiObjectReferenceKind = 4;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_SIGNERS: jvmtiObjectReferenceKind = 5;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_PROTECTION_DOMAIN: jvmtiObjectReferenceKind = 6;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_INTERFACE: jvmtiObjectReferenceKind = 7;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_STATIC_FIELD: jvmtiObjectReferenceKind = 8;
pub const jvmtiObjectReferenceKind_JVMTI_REFERENCE_CONSTANT_POOL: jvmtiObjectReferenceKind = 9;

pub type jvmtiObjectReferenceKind = u32;

pub const jvmtiIterationControl_JVMTI_ITERATION_CONTINUE: jvmtiIterationControl = 1;
pub const jvmtiIterationControl_JVMTI_ITERATION_IGNORE: jvmtiIterationControl = 2;
pub const jvmtiIterationControl_JVMTI_ITERATION_ABORT: jvmtiIterationControl = 0;

pub type jvmtiIterationControl = u32;

pub const JVMTI_CLASS_STATUS_VERIFIED: _bindgen_ty_7 = 1;
pub const JVMTI_CLASS_STATUS_PREPARED: _bindgen_ty_7 = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED: _bindgen_ty_7 = 4;
pub const JVMTI_CLASS_STATUS_ERROR: _bindgen_ty_7 = 8;
pub const JVMTI_CLASS_STATUS_ARRAY: _bindgen_ty_7 = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE: _bindgen_ty_7 = 32;

pub type _bindgen_ty_7 = u32;

pub const jvmtiEventMode_JVMTI_ENABLE: jvmtiEventMode = 1;
pub const jvmtiEventMode_JVMTI_DISABLE: jvmtiEventMode = 0;

pub type jvmtiEventMode = u32;

pub const jvmtiParamTypes_JVMTI_TYPE_JBYTE: jvmtiParamTypes = 101;
pub const jvmtiParamTypes_JVMTI_TYPE_JCHAR: jvmtiParamTypes = 102;
pub const jvmtiParamTypes_JVMTI_TYPE_JSHORT: jvmtiParamTypes = 103;
pub const jvmtiParamTypes_JVMTI_TYPE_JINT: jvmtiParamTypes = 104;
pub const jvmtiParamTypes_JVMTI_TYPE_JLONG: jvmtiParamTypes = 105;
pub const jvmtiParamTypes_JVMTI_TYPE_JFLOAT: jvmtiParamTypes = 106;
pub const jvmtiParamTypes_JVMTI_TYPE_JDOUBLE: jvmtiParamTypes = 107;
pub const jvmtiParamTypes_JVMTI_TYPE_JBOOLEAN: jvmtiParamTypes = 108;
pub const jvmtiParamTypes_JVMTI_TYPE_JOBJECT: jvmtiParamTypes = 109;
pub const jvmtiParamTypes_JVMTI_TYPE_JTHREAD: jvmtiParamTypes = 110;
pub const jvmtiParamTypes_JVMTI_TYPE_JCLASS: jvmtiParamTypes = 111;
pub const jvmtiParamTypes_JVMTI_TYPE_JVALUE: jvmtiParamTypes = 112;
pub const jvmtiParamTypes_JVMTI_TYPE_JFIELDID: jvmtiParamTypes = 113;
pub const jvmtiParamTypes_JVMTI_TYPE_JMETHODID: jvmtiParamTypes = 114;
pub const jvmtiParamTypes_JVMTI_TYPE_CCHAR: jvmtiParamTypes = 115;
pub const jvmtiParamTypes_JVMTI_TYPE_CVOID: jvmtiParamTypes = 116;
pub const jvmtiParamTypes_JVMTI_TYPE_JNIENV: jvmtiParamTypes = 117;

pub type jvmtiParamTypes = u32;

pub const jvmtiParamKind_JVMTI_KIND_IN: jvmtiParamKind = 91;
pub const jvmtiParamKind_JVMTI_KIND_IN_PTR: jvmtiParamKind = 92;
pub const jvmtiParamKind_JVMTI_KIND_IN_BUF: jvmtiParamKind = 93;
pub const jvmtiParamKind_JVMTI_KIND_ALLOC_BUF: jvmtiParamKind = 94;
pub const jvmtiParamKind_JVMTI_KIND_ALLOC_ALLOC_BUF: jvmtiParamKind = 95;
pub const jvmtiParamKind_JVMTI_KIND_OUT: jvmtiParamKind = 96;
pub const jvmtiParamKind_JVMTI_KIND_OUT_BUF: jvmtiParamKind = 97;

pub type jvmtiParamKind = u32;

pub const jvmtiTimerKind_JVMTI_TIMER_USER_CPU: jvmtiTimerKind = 30;
pub const jvmtiTimerKind_JVMTI_TIMER_TOTAL_CPU: jvmtiTimerKind = 31;
pub const jvmtiTimerKind_JVMTI_TIMER_ELAPSED: jvmtiTimerKind = 32;

pub type jvmtiTimerKind = u32;

pub const jvmtiPhase_JVMTI_PHASE_ONLOAD: jvmtiPhase = 1;
pub const jvmtiPhase_JVMTI_PHASE_PRIMORDIAL: jvmtiPhase = 2;
pub const jvmtiPhase_JVMTI_PHASE_START: jvmtiPhase = 6;
pub const jvmtiPhase_JVMTI_PHASE_LIVE: jvmtiPhase = 4;
pub const jvmtiPhase_JVMTI_PHASE_DEAD: jvmtiPhase = 8;

pub type jvmtiPhase = u32;

pub const JVMTI_VERSION_INTERFACE_JNI: _bindgen_ty_8 = 0;
pub const JVMTI_VERSION_INTERFACE_JVMTI: _bindgen_ty_8 = 805306368;

pub type _bindgen_ty_8 = u32;

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: _bindgen_ty_9 = 1879048192;
pub const JVMTI_VERSION_MASK_MAJOR: _bindgen_ty_9 = 268369920;
pub const JVMTI_VERSION_MASK_MINOR: _bindgen_ty_9 = 65280;
pub const JVMTI_VERSION_MASK_MICRO: _bindgen_ty_9 = 255;

pub type _bindgen_ty_9 = u32;

pub const JVMTI_VERSION_SHIFT_MAJOR: _bindgen_ty_10 = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: _bindgen_ty_10 = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: _bindgen_ty_10 = 0;

pub type _bindgen_ty_10 = u32;

pub const jvmtiVerboseFlag_JVMTI_VERBOSE_OTHER: jvmtiVerboseFlag = 0;
pub const jvmtiVerboseFlag_JVMTI_VERBOSE_GC: jvmtiVerboseFlag = 1;
pub const jvmtiVerboseFlag_JVMTI_VERBOSE_CLASS: jvmtiVerboseFlag = 2;
pub const jvmtiVerboseFlag_JVMTI_VERBOSE_JNI: jvmtiVerboseFlag = 4;

pub type jvmtiVerboseFlag = u32;

pub const jvmtiJlocationFormat_JVMTI_JLOCATION_JVMBCI: jvmtiJlocationFormat = 1;
pub const jvmtiJlocationFormat_JVMTI_JLOCATION_MACHINEPC: jvmtiJlocationFormat = 2;
pub const jvmtiJlocationFormat_JVMTI_JLOCATION_OTHER: jvmtiJlocationFormat = 0;

pub type jvmtiJlocationFormat = u32;

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: _bindgen_ty_11 = 1;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: _bindgen_ty_11 = 2;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: _bindgen_ty_11 = 4;

pub type _bindgen_ty_11 = u32;

pub const jvmtiError_JVMTI_ERROR_NONE: jvmtiError = 0;
pub const jvmtiError_JVMTI_ERROR_INVALID_THREAD: jvmtiError = 10;
pub const jvmtiError_JVMTI_ERROR_INVALID_THREAD_GROUP: jvmtiError = 11;
pub const jvmtiError_JVMTI_ERROR_INVALID_PRIORITY: jvmtiError = 12;
pub const jvmtiError_JVMTI_ERROR_THREAD_NOT_SUSPENDED: jvmtiError = 13;
pub const jvmtiError_JVMTI_ERROR_THREAD_SUSPENDED: jvmtiError = 14;
pub const jvmtiError_JVMTI_ERROR_THREAD_NOT_ALIVE: jvmtiError = 15;
pub const jvmtiError_JVMTI_ERROR_INVALID_OBJECT: jvmtiError = 20;
pub const jvmtiError_JVMTI_ERROR_INVALID_CLASS: jvmtiError = 21;
pub const jvmtiError_JVMTI_ERROR_CLASS_NOT_PREPARED: jvmtiError = 22;
pub const jvmtiError_JVMTI_ERROR_INVALID_METHODID: jvmtiError = 23;
pub const jvmtiError_JVMTI_ERROR_INVALID_LOCATION: jvmtiError = 24;
pub const jvmtiError_JVMTI_ERROR_INVALID_FIELDID: jvmtiError = 25;
pub const jvmtiError_JVMTI_ERROR_NO_MORE_FRAMES: jvmtiError = 31;
pub const jvmtiError_JVMTI_ERROR_OPAQUE_FRAME: jvmtiError = 32;
pub const jvmtiError_JVMTI_ERROR_TYPE_MISMATCH: jvmtiError = 34;
pub const jvmtiError_JVMTI_ERROR_INVALID_SLOT: jvmtiError = 35;
pub const jvmtiError_JVMTI_ERROR_DUPLICATE: jvmtiError = 40;
pub const jvmtiError_JVMTI_ERROR_NOT_FOUND: jvmtiError = 41;
pub const jvmtiError_JVMTI_ERROR_INVALID_MONITOR: jvmtiError = 50;
pub const jvmtiError_JVMTI_ERROR_NOT_MONITOR_OWNER: jvmtiError = 51;
pub const jvmtiError_JVMTI_ERROR_INTERRUPT: jvmtiError = 52;
pub const jvmtiError_JVMTI_ERROR_INVALID_CLASS_FORMAT: jvmtiError = 60;
pub const jvmtiError_JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION: jvmtiError = 61;
pub const jvmtiError_JVMTI_ERROR_FAILS_VERIFICATION: jvmtiError = 62;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED: jvmtiError = 63;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED: jvmtiError = 64;
pub const jvmtiError_JVMTI_ERROR_INVALID_TYPESTATE: jvmtiError = 65;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED: jvmtiError = 66;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED: jvmtiError = 67;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_VERSION: jvmtiError = 68;
pub const jvmtiError_JVMTI_ERROR_NAMES_DONT_MATCH: jvmtiError = 69;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED: jvmtiError = 70;
pub const jvmtiError_JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED: jvmtiError = 71;
pub const jvmtiError_JVMTI_ERROR_UNMODIFIABLE_CLASS: jvmtiError = 79;
pub const jvmtiError_JVMTI_ERROR_NOT_AVAILABLE: jvmtiError = 98;
pub const jvmtiError_JVMTI_ERROR_MUST_POSSESS_CAPABILITY: jvmtiError = 99;
pub const jvmtiError_JVMTI_ERROR_NULL_POINTER: jvmtiError = 100;
pub const jvmtiError_JVMTI_ERROR_ABSENT_INFORMATION: jvmtiError = 101;
pub const jvmtiError_JVMTI_ERROR_INVALID_EVENT_TYPE: jvmtiError = 102;
pub const jvmtiError_JVMTI_ERROR_ILLEGAL_ARGUMENT: jvmtiError = 103;
pub const jvmtiError_JVMTI_ERROR_NATIVE_METHOD: jvmtiError = 104;
pub const jvmtiError_JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED: jvmtiError = 106;
pub const jvmtiError_JVMTI_ERROR_OUT_OF_MEMORY: jvmtiError = 110;
pub const jvmtiError_JVMTI_ERROR_ACCESS_DENIED: jvmtiError = 111;
pub const jvmtiError_JVMTI_ERROR_WRONG_PHASE: jvmtiError = 112;
pub const jvmtiError_JVMTI_ERROR_INTERNAL: jvmtiError = 113;
pub const jvmtiError_JVMTI_ERROR_UNATTACHED_THREAD: jvmtiError = 115;
pub const jvmtiError_JVMTI_ERROR_INVALID_ENVIRONMENT: jvmtiError = 116;
pub const jvmtiError_JVMTI_ERROR_MAX: jvmtiError = 116;

pub type jvmtiError = u32;

pub const jvmtiEvent_JVMTI_MIN_EVENT_TYPE_VAL: jvmtiEvent = 50;
pub const jvmtiEvent_JVMTI_EVENT_VM_INIT: jvmtiEvent = 50;
pub const jvmtiEvent_JVMTI_EVENT_VM_DEATH: jvmtiEvent = 51;
pub const jvmtiEvent_JVMTI_EVENT_THREAD_START: jvmtiEvent = 52;
pub const jvmtiEvent_JVMTI_EVENT_THREAD_END: jvmtiEvent = 53;
pub const jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK: jvmtiEvent = 54;
pub const jvmtiEvent_JVMTI_EVENT_CLASS_LOAD: jvmtiEvent = 55;
pub const jvmtiEvent_JVMTI_EVENT_CLASS_PREPARE: jvmtiEvent = 56;
pub const jvmtiEvent_JVMTI_EVENT_VM_START: jvmtiEvent = 57;
pub const jvmtiEvent_JVMTI_EVENT_EXCEPTION: jvmtiEvent = 58;
pub const jvmtiEvent_JVMTI_EVENT_EXCEPTION_CATCH: jvmtiEvent = 59;
pub const jvmtiEvent_JVMTI_EVENT_SINGLE_STEP: jvmtiEvent = 60;
pub const jvmtiEvent_JVMTI_EVENT_FRAME_POP: jvmtiEvent = 61;
pub const jvmtiEvent_JVMTI_EVENT_BREAKPOINT: jvmtiEvent = 62;
pub const jvmtiEvent_JVMTI_EVENT_FIELD_ACCESS: jvmtiEvent = 63;
pub const jvmtiEvent_JVMTI_EVENT_FIELD_MODIFICATION: jvmtiEvent = 64;
pub const jvmtiEvent_JVMTI_EVENT_METHOD_ENTRY: jvmtiEvent = 65;
pub const jvmtiEvent_JVMTI_EVENT_METHOD_EXIT: jvmtiEvent = 66;
pub const jvmtiEvent_JVMTI_EVENT_NATIVE_METHOD_BIND: jvmtiEvent = 67;
pub const jvmtiEvent_JVMTI_EVENT_COMPILED_METHOD_LOAD: jvmtiEvent = 68;
pub const jvmtiEvent_JVMTI_EVENT_COMPILED_METHOD_UNLOAD: jvmtiEvent = 69;
pub const jvmtiEvent_JVMTI_EVENT_DYNAMIC_CODE_GENERATED: jvmtiEvent = 70;
pub const jvmtiEvent_JVMTI_EVENT_DATA_DUMP_REQUEST: jvmtiEvent = 71;
pub const jvmtiEvent_JVMTI_EVENT_MONITOR_WAIT: jvmtiEvent = 73;
pub const jvmtiEvent_JVMTI_EVENT_MONITOR_WAITED: jvmtiEvent = 74;
pub const jvmtiEvent_JVMTI_EVENT_MONITOR_CONTENDED_ENTER: jvmtiEvent = 75;
pub const jvmtiEvent_JVMTI_EVENT_MONITOR_CONTENDED_ENTERED: jvmtiEvent = 76;
pub const jvmtiEvent_JVMTI_EVENT_RESOURCE_EXHAUSTED: jvmtiEvent = 80;
pub const jvmtiEvent_JVMTI_EVENT_GARBAGE_COLLECTION_START: jvmtiEvent = 81;
pub const jvmtiEvent_JVMTI_EVENT_GARBAGE_COLLECTION_FINISH: jvmtiEvent = 82;
pub const jvmtiEvent_JVMTI_EVENT_OBJECT_FREE: jvmtiEvent = 83;
pub const jvmtiEvent_JVMTI_EVENT_VM_OBJECT_ALLOC: jvmtiEvent = 84;
pub const jvmtiEvent_JVMTI_MAX_EVENT_TYPE_VAL: jvmtiEvent = 84;

pub type jvmtiEvent = u32;
pub type jvmtiThreadInfo = _jvmtiThreadInfo;
pub type jvmtiMonitorStackDepthInfo = _jvmtiMonitorStackDepthInfo;
pub type jvmtiThreadGroupInfo = _jvmtiThreadGroupInfo;
pub type jvmtiFrameInfo = _jvmtiFrameInfo;
pub type jvmtiStackInfo = _jvmtiStackInfo;
pub type jvmtiHeapReferenceInfoField = _jvmtiHeapReferenceInfoField;
pub type jvmtiHeapReferenceInfoArray = _jvmtiHeapReferenceInfoArray;
pub type jvmtiHeapReferenceInfoConstantPool = _jvmtiHeapReferenceInfoConstantPool;
pub type jvmtiHeapReferenceInfoStackLocal = _jvmtiHeapReferenceInfoStackLocal;
pub type jvmtiHeapReferenceInfoJniLocal = _jvmtiHeapReferenceInfoJniLocal;
pub type jvmtiHeapReferenceInfoReserved = _jvmtiHeapReferenceInfoReserved;
pub type jvmtiHeapReferenceInfo = _jvmtiHeapReferenceInfo;
pub type jvmtiHeapCallbacks = _jvmtiHeapCallbacks;
pub type jvmtiClassDefinition = _jvmtiClassDefinition;
pub type jvmtiMonitorUsage = _jvmtiMonitorUsage;
pub type jvmtiLineNumberEntry = _jvmtiLineNumberEntry;
pub type jvmtiLocalVariableEntry = _jvmtiLocalVariableEntry;
pub type jvmtiParamInfo = _jvmtiParamInfo;
pub type jvmtiExtensionFunctionInfo = _jvmtiExtensionFunctionInfo;
pub type jvmtiExtensionEventInfo = _jvmtiExtensionEventInfo;
pub type jvmtiTimerInfo = _jvmtiTimerInfo;
pub type jvmtiAddrLocationMap = _jvmtiAddrLocationMap;
pub type jvmtiStartFunction = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		arg: *mut ::std::os::raw::c_void,
	),
>;
pub type jvmtiHeapIterationCallback = ::std::option::Option<
	unsafe extern "C" fn(
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		length: jint,
		user_data: *mut ::std::os::raw::c_void,
	) -> jint,
>;
pub type jvmtiHeapReferenceCallback = ::std::option::Option<
	unsafe extern "C" fn(
		reference_kind: jvmtiHeapReferenceKind,
		reference_info: *const jvmtiHeapReferenceInfo,
		class_tag: jlong,
		referrer_class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		referrer_tag_ptr: *mut jlong,
		length: jint,
		user_data: *mut ::std::os::raw::c_void,
	) -> jint,
>;
pub type jvmtiPrimitiveFieldCallback = ::std::option::Option<
	unsafe extern "C" fn(
		kind: jvmtiHeapReferenceKind,
		info: *const jvmtiHeapReferenceInfo,
		object_class_tag: jlong,
		object_tag_ptr: *mut jlong,
		value: jvalue,
		value_type: jvmtiPrimitiveType,
		user_data: *mut ::std::os::raw::c_void,
	) -> jint,
>;
pub type jvmtiArrayPrimitiveValueCallback = ::std::option::Option<
	unsafe extern "C" fn(
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		element_count: jint,
		element_type: jvmtiPrimitiveType,
		elements: *const ::std::os::raw::c_void,
		user_data: *mut ::std::os::raw::c_void,
	) -> jint,
>;
pub type jvmtiStringPrimitiveValueCallback = ::std::option::Option<
	unsafe extern "C" fn(
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		value: *const jchar,
		value_length: jint,
		user_data: *mut ::std::os::raw::c_void,
	) -> jint,
>;
pub type jvmtiReservedCallback = ::std::option::Option<unsafe extern "C" fn() -> jint>;
pub type jvmtiHeapObjectCallback = ::std::option::Option<
	unsafe extern "C" fn(
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		user_data: *mut ::std::os::raw::c_void,
	) -> jvmtiIterationControl,
>;
pub type jvmtiHeapRootCallback = ::std::option::Option<
	unsafe extern "C" fn(
		root_kind: jvmtiHeapRootKind,
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		user_data: *mut ::std::os::raw::c_void,
	) -> jvmtiIterationControl,
>;
pub type jvmtiStackReferenceCallback = ::std::option::Option<
	unsafe extern "C" fn(
		root_kind: jvmtiHeapRootKind,
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		thread_tag: jlong,
		depth: jint,
		method: jmethodID,
		slot: jint,
		user_data: *mut ::std::os::raw::c_void,
	) -> jvmtiIterationControl,
>;
pub type jvmtiObjectReferenceCallback = ::std::option::Option<
	unsafe extern "C" fn(
		reference_kind: jvmtiObjectReferenceKind,
		class_tag: jlong,
		size: jlong,
		tag_ptr: *mut jlong,
		referrer_tag: jlong,
		referrer_index: jint,
		user_data: *mut ::std::os::raw::c_void,
	) -> jvmtiIterationControl,
>;
pub type jvmtiExtensionFunction =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError>;
pub type jvmtiExtensionEvent =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiThreadInfo {
	pub name: *mut ::std::os::raw::c_char,
	pub priority: jint,
	pub is_daemon: jboolean,
	pub thread_group: jthreadGroup,
	pub context_class_loader: jobject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiMonitorStackDepthInfo {
	pub monitor: jobject,
	pub stack_depth: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiThreadGroupInfo {
	pub parent: jthreadGroup,
	pub name: *mut ::std::os::raw::c_char,
	pub max_priority: jint,
	pub is_daemon: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiFrameInfo {
	pub method: jmethodID,
	pub location: jlocation,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiStackInfo {
	pub thread: jthread,
	pub state: jint,
	pub frame_buffer: *mut jvmtiFrameInfo,
	pub frame_count: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoField {
	pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoArray {
	pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoConstantPool {
	pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoStackLocal {
	pub thread_tag: jlong,
	pub thread_id: jlong,
	pub depth: jint,
	pub method: jmethodID,
	pub location: jlocation,
	pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoJniLocal {
	pub thread_tag: jlong,
	pub thread_id: jlong,
	pub depth: jint,
	pub method: jmethodID,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoReserved {
	pub reserved1: jlong,
	pub reserved2: jlong,
	pub reserved3: jlong,
	pub reserved4: jlong,
	pub reserved5: jlong,
	pub reserved6: jlong,
	pub reserved7: jlong,
	pub reserved8: jlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _jvmtiHeapReferenceInfo {
	pub field: jvmtiHeapReferenceInfoField,
	pub array: jvmtiHeapReferenceInfoArray,
	pub constant_pool: jvmtiHeapReferenceInfoConstantPool,
	pub stack_local: jvmtiHeapReferenceInfoStackLocal,
	pub jni_local: jvmtiHeapReferenceInfoJniLocal,
	pub other: jvmtiHeapReferenceInfoReserved,
	_bindgen_union_align: [u64; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapCallbacks {
	pub heap_iteration_callback: jvmtiHeapIterationCallback,
	pub heap_reference_callback: jvmtiHeapReferenceCallback,
	pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
	pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
	pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
	pub reserved5: jvmtiReservedCallback,
	pub reserved6: jvmtiReservedCallback,
	pub reserved7: jvmtiReservedCallback,
	pub reserved8: jvmtiReservedCallback,
	pub reserved9: jvmtiReservedCallback,
	pub reserved10: jvmtiReservedCallback,
	pub reserved11: jvmtiReservedCallback,
	pub reserved12: jvmtiReservedCallback,
	pub reserved13: jvmtiReservedCallback,
	pub reserved14: jvmtiReservedCallback,
	pub reserved15: jvmtiReservedCallback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiClassDefinition {
	pub klass: jclass,
	pub class_byte_count: jint,
	pub class_bytes: *const ::std::os::raw::c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiMonitorUsage {
	pub owner: jthread,
	pub entry_count: jint,
	pub waiter_count: jint,
	pub waiters: *mut jthread,
	pub notify_waiter_count: jint,
	pub notify_waiters: *mut jthread,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiLineNumberEntry {
	pub start_location: jlocation,
	pub line_number: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiLocalVariableEntry {
	pub start_location: jlocation,
	pub length: jint,
	pub name: *mut ::std::os::raw::c_char,
	pub signature: *mut ::std::os::raw::c_char,
	pub generic_signature: *mut ::std::os::raw::c_char,
	pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiParamInfo {
	pub name: *mut ::std::os::raw::c_char,
	pub kind: jvmtiParamKind,
	pub base_type: jvmtiParamTypes,
	pub null_ok: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiExtensionFunctionInfo {
	pub func: jvmtiExtensionFunction,
	pub id: *mut ::std::os::raw::c_char,
	pub short_description: *mut ::std::os::raw::c_char,
	pub param_count: jint,
	pub params: *mut jvmtiParamInfo,
	pub error_count: jint,
	pub errors: *mut jvmtiError,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiExtensionEventInfo {
	pub extension_event_index: jint,
	pub id: *mut ::std::os::raw::c_char,
	pub short_description: *mut ::std::os::raw::c_char,
	pub param_count: jint,
	pub params: *mut jvmtiParamInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiTimerInfo {
	pub max_value: jlong,
	pub may_skip_forward: jboolean,
	pub may_skip_backward: jboolean,
	pub kind: jvmtiTimerKind,
	pub reserved1: jlong,
	pub reserved2: jlong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiAddrLocationMap {
	pub start_address: *const ::std::os::raw::c_void,
	pub location: jlocation,
}

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiCapabilities {
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize], u8>,
}

impl jvmtiCapabilities {
	#[inline]
	pub fn can_tag_objects(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_tag_objects(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_field_modification_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_field_modification_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_field_access_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_field_access_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_bytecodes(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_bytecodes(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(3usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_synthetic_attribute(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_synthetic_attribute(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(4usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_owned_monitor_info(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_owned_monitor_info(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(5usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_current_contended_monitor(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_current_contended_monitor(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(6usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_monitor_info(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_monitor_info(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(7usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_pop_frame(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_pop_frame(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(8usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_redefine_classes(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_redefine_classes(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(9usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_signal_thread(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_signal_thread(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(10usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_source_file_name(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_source_file_name(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(11usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_line_numbers(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_line_numbers(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(12usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_source_debug_extension(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_source_debug_extension(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(13usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_access_local_variables(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_access_local_variables(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(14usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_maintain_original_method_order(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_maintain_original_method_order(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(15usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_single_step_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_single_step_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(16usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_exception_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_exception_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(17usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_frame_pop_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_frame_pop_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(18usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_breakpoint_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_breakpoint_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(19usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_suspend(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_suspend(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(20usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_redefine_any_class(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_redefine_any_class(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(21usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_current_thread_cpu_time(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_current_thread_cpu_time(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(22usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_thread_cpu_time(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_thread_cpu_time(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(23usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_method_entry_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_method_entry_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(24usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_method_exit_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_method_exit_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(25usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_all_class_hook_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_all_class_hook_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(26usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_compiled_method_load_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_compiled_method_load_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(27usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_monitor_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_monitor_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(28usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_vm_object_alloc_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_vm_object_alloc_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(29usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_native_method_bind_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_native_method_bind_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(30usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_garbage_collection_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_garbage_collection_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(31usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_object_free_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_object_free_events(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(32usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_force_early_return(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_force_early_return(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(33usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_owned_monitor_stack_depth_info(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(34usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_owned_monitor_stack_depth_info(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(34usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_get_constant_pool(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(35usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_get_constant_pool(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(35usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_set_native_method_prefix(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(36usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_set_native_method_prefix(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(36usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_retransform_classes(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(37usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_retransform_classes(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(37usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_retransform_any_class(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(38usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_retransform_any_class(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(38usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_resource_exhaustion_heap_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(39usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_resource_exhaustion_heap_events(
		&mut self,
		val: ::std::os::raw::c_uint,
	) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(39usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn can_generate_resource_exhaustion_threads_events(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(40usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_can_generate_resource_exhaustion_threads_events(
		&mut self,
		val: ::std::os::raw::c_uint,
	) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(40usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_1(
		can_tag_objects: ::std::os::raw::c_uint,
		can_generate_field_modification_events: ::std::os::raw::c_uint,
		can_generate_field_access_events: ::std::os::raw::c_uint,
		can_get_bytecodes: ::std::os::raw::c_uint,
		can_get_synthetic_attribute: ::std::os::raw::c_uint,
		can_get_owned_monitor_info: ::std::os::raw::c_uint,
		can_get_current_contended_monitor: ::std::os::raw::c_uint,
		can_get_monitor_info: ::std::os::raw::c_uint,
		can_pop_frame: ::std::os::raw::c_uint,
		can_redefine_classes: ::std::os::raw::c_uint,
		can_signal_thread: ::std::os::raw::c_uint,
		can_get_source_file_name: ::std::os::raw::c_uint,
		can_get_line_numbers: ::std::os::raw::c_uint,
		can_get_source_debug_extension: ::std::os::raw::c_uint,
		can_access_local_variables: ::std::os::raw::c_uint,
		can_maintain_original_method_order: ::std::os::raw::c_uint,
		can_generate_single_step_events: ::std::os::raw::c_uint,
		can_generate_exception_events: ::std::os::raw::c_uint,
		can_generate_frame_pop_events: ::std::os::raw::c_uint,
		can_generate_breakpoint_events: ::std::os::raw::c_uint,
		can_suspend: ::std::os::raw::c_uint,
		can_redefine_any_class: ::std::os::raw::c_uint,
		can_get_current_thread_cpu_time: ::std::os::raw::c_uint,
		can_get_thread_cpu_time: ::std::os::raw::c_uint,
		can_generate_method_entry_events: ::std::os::raw::c_uint,
		can_generate_method_exit_events: ::std::os::raw::c_uint,
		can_generate_all_class_hook_events: ::std::os::raw::c_uint,
		can_generate_compiled_method_load_events: ::std::os::raw::c_uint,
		can_generate_monitor_events: ::std::os::raw::c_uint,
		can_generate_vm_object_alloc_events: ::std::os::raw::c_uint,
		can_generate_native_method_bind_events: ::std::os::raw::c_uint,
		can_generate_garbage_collection_events: ::std::os::raw::c_uint,
		can_generate_object_free_events: ::std::os::raw::c_uint,
		can_force_early_return: ::std::os::raw::c_uint,
		can_get_owned_monitor_stack_depth_info: ::std::os::raw::c_uint,
		can_get_constant_pool: ::std::os::raw::c_uint,
		can_set_native_method_prefix: ::std::os::raw::c_uint,
		can_retransform_classes: ::std::os::raw::c_uint,
		can_retransform_any_class: ::std::os::raw::c_uint,
		can_generate_resource_exhaustion_heap_events: ::std::os::raw::c_uint,
		can_generate_resource_exhaustion_threads_events: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 16usize], u8> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize], u8> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let can_tag_objects: u32 = unsafe { ::std::mem::transmute(can_tag_objects) };
			can_tag_objects as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let can_generate_field_modification_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_field_modification_events) };
			can_generate_field_modification_events as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let can_generate_field_access_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_field_access_events) };
			can_generate_field_access_events as u64
		});
		__bindgen_bitfield_unit.set(3usize, 1u8, {
			let can_get_bytecodes: u32 = unsafe { ::std::mem::transmute(can_get_bytecodes) };
			can_get_bytecodes as u64
		});
		__bindgen_bitfield_unit.set(4usize, 1u8, {
			let can_get_synthetic_attribute: u32 =
				unsafe { ::std::mem::transmute(can_get_synthetic_attribute) };
			can_get_synthetic_attribute as u64
		});
		__bindgen_bitfield_unit.set(5usize, 1u8, {
			let can_get_owned_monitor_info: u32 =
				unsafe { ::std::mem::transmute(can_get_owned_monitor_info) };
			can_get_owned_monitor_info as u64
		});
		__bindgen_bitfield_unit.set(6usize, 1u8, {
			let can_get_current_contended_monitor: u32 =
				unsafe { ::std::mem::transmute(can_get_current_contended_monitor) };
			can_get_current_contended_monitor as u64
		});
		__bindgen_bitfield_unit.set(7usize, 1u8, {
			let can_get_monitor_info: u32 = unsafe { ::std::mem::transmute(can_get_monitor_info) };
			can_get_monitor_info as u64
		});
		__bindgen_bitfield_unit.set(8usize, 1u8, {
			let can_pop_frame: u32 = unsafe { ::std::mem::transmute(can_pop_frame) };
			can_pop_frame as u64
		});
		__bindgen_bitfield_unit.set(9usize, 1u8, {
			let can_redefine_classes: u32 = unsafe { ::std::mem::transmute(can_redefine_classes) };
			can_redefine_classes as u64
		});
		__bindgen_bitfield_unit.set(10usize, 1u8, {
			let can_signal_thread: u32 = unsafe { ::std::mem::transmute(can_signal_thread) };
			can_signal_thread as u64
		});
		__bindgen_bitfield_unit.set(11usize, 1u8, {
			let can_get_source_file_name: u32 =
				unsafe { ::std::mem::transmute(can_get_source_file_name) };
			can_get_source_file_name as u64
		});
		__bindgen_bitfield_unit.set(12usize, 1u8, {
			let can_get_line_numbers: u32 = unsafe { ::std::mem::transmute(can_get_line_numbers) };
			can_get_line_numbers as u64
		});
		__bindgen_bitfield_unit.set(13usize, 1u8, {
			let can_get_source_debug_extension: u32 =
				unsafe { ::std::mem::transmute(can_get_source_debug_extension) };
			can_get_source_debug_extension as u64
		});
		__bindgen_bitfield_unit.set(14usize, 1u8, {
			let can_access_local_variables: u32 =
				unsafe { ::std::mem::transmute(can_access_local_variables) };
			can_access_local_variables as u64
		});
		__bindgen_bitfield_unit.set(15usize, 1u8, {
			let can_maintain_original_method_order: u32 =
				unsafe { ::std::mem::transmute(can_maintain_original_method_order) };
			can_maintain_original_method_order as u64
		});
		__bindgen_bitfield_unit.set(16usize, 1u8, {
			let can_generate_single_step_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_single_step_events) };
			can_generate_single_step_events as u64
		});
		__bindgen_bitfield_unit.set(17usize, 1u8, {
			let can_generate_exception_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_exception_events) };
			can_generate_exception_events as u64
		});
		__bindgen_bitfield_unit.set(18usize, 1u8, {
			let can_generate_frame_pop_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_frame_pop_events) };
			can_generate_frame_pop_events as u64
		});
		__bindgen_bitfield_unit.set(19usize, 1u8, {
			let can_generate_breakpoint_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_breakpoint_events) };
			can_generate_breakpoint_events as u64
		});
		__bindgen_bitfield_unit.set(20usize, 1u8, {
			let can_suspend: u32 = unsafe { ::std::mem::transmute(can_suspend) };
			can_suspend as u64
		});
		__bindgen_bitfield_unit.set(21usize, 1u8, {
			let can_redefine_any_class: u32 =
				unsafe { ::std::mem::transmute(can_redefine_any_class) };
			can_redefine_any_class as u64
		});
		__bindgen_bitfield_unit.set(22usize, 1u8, {
			let can_get_current_thread_cpu_time: u32 =
				unsafe { ::std::mem::transmute(can_get_current_thread_cpu_time) };
			can_get_current_thread_cpu_time as u64
		});
		__bindgen_bitfield_unit.set(23usize, 1u8, {
			let can_get_thread_cpu_time: u32 =
				unsafe { ::std::mem::transmute(can_get_thread_cpu_time) };
			can_get_thread_cpu_time as u64
		});
		__bindgen_bitfield_unit.set(24usize, 1u8, {
			let can_generate_method_entry_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_method_entry_events) };
			can_generate_method_entry_events as u64
		});
		__bindgen_bitfield_unit.set(25usize, 1u8, {
			let can_generate_method_exit_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_method_exit_events) };
			can_generate_method_exit_events as u64
		});
		__bindgen_bitfield_unit.set(26usize, 1u8, {
			let can_generate_all_class_hook_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_all_class_hook_events) };
			can_generate_all_class_hook_events as u64
		});
		__bindgen_bitfield_unit.set(27usize, 1u8, {
			let can_generate_compiled_method_load_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_compiled_method_load_events) };
			can_generate_compiled_method_load_events as u64
		});
		__bindgen_bitfield_unit.set(28usize, 1u8, {
			let can_generate_monitor_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_monitor_events) };
			can_generate_monitor_events as u64
		});
		__bindgen_bitfield_unit.set(29usize, 1u8, {
			let can_generate_vm_object_alloc_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_vm_object_alloc_events) };
			can_generate_vm_object_alloc_events as u64
		});
		__bindgen_bitfield_unit.set(30usize, 1u8, {
			let can_generate_native_method_bind_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_native_method_bind_events) };
			can_generate_native_method_bind_events as u64
		});
		__bindgen_bitfield_unit.set(31usize, 1u8, {
			let can_generate_garbage_collection_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_garbage_collection_events) };
			can_generate_garbage_collection_events as u64
		});
		__bindgen_bitfield_unit.set(32usize, 1u8, {
			let can_generate_object_free_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_object_free_events) };
			can_generate_object_free_events as u64
		});
		__bindgen_bitfield_unit.set(33usize, 1u8, {
			let can_force_early_return: u32 =
				unsafe { ::std::mem::transmute(can_force_early_return) };
			can_force_early_return as u64
		});
		__bindgen_bitfield_unit.set(34usize, 1u8, {
			let can_get_owned_monitor_stack_depth_info: u32 =
				unsafe { ::std::mem::transmute(can_get_owned_monitor_stack_depth_info) };
			can_get_owned_monitor_stack_depth_info as u64
		});
		__bindgen_bitfield_unit.set(35usize, 1u8, {
			let can_get_constant_pool: u32 =
				unsafe { ::std::mem::transmute(can_get_constant_pool) };
			can_get_constant_pool as u64
		});
		__bindgen_bitfield_unit.set(36usize, 1u8, {
			let can_set_native_method_prefix: u32 =
				unsafe { ::std::mem::transmute(can_set_native_method_prefix) };
			can_set_native_method_prefix as u64
		});
		__bindgen_bitfield_unit.set(37usize, 1u8, {
			let can_retransform_classes: u32 =
				unsafe { ::std::mem::transmute(can_retransform_classes) };
			can_retransform_classes as u64
		});
		__bindgen_bitfield_unit.set(38usize, 1u8, {
			let can_retransform_any_class: u32 =
				unsafe { ::std::mem::transmute(can_retransform_any_class) };
			can_retransform_any_class as u64
		});
		__bindgen_bitfield_unit.set(39usize, 1u8, {
			let can_generate_resource_exhaustion_heap_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_resource_exhaustion_heap_events) };
			can_generate_resource_exhaustion_heap_events as u64
		});
		__bindgen_bitfield_unit.set(40usize, 1u8, {
			let can_generate_resource_exhaustion_threads_events: u32 =
				unsafe { ::std::mem::transmute(can_generate_resource_exhaustion_threads_events) };
			can_generate_resource_exhaustion_threads_events as u64
		});
		__bindgen_bitfield_unit
	}
}

pub type jvmtiEventReserved = ::std::option::Option<unsafe extern "C" fn()>;
pub type jvmtiEventBreakpoint = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
	),
>;
pub type jvmtiEventClassFileLoadHook = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		class_being_redefined: jclass,
		loader: jobject,
		name: *const ::std::os::raw::c_char,
		protection_domain: jobject,
		class_data_len: jint,
		class_data: *const ::std::os::raw::c_uchar,
		new_class_data_len: *mut jint,
		new_class_data: *mut *mut ::std::os::raw::c_uchar,
	),
>;
pub type jvmtiEventClassLoad = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		klass: jclass,
	),
>;
pub type jvmtiEventClassPrepare = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		klass: jclass,
	),
>;
pub type jvmtiEventCompiledMethodLoad = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		method: jmethodID,
		code_size: jint,
		code_addr: *const ::std::os::raw::c_void,
		map_length: jint,
		map: *const jvmtiAddrLocationMap,
		compile_info: *const ::std::os::raw::c_void,
	),
>;
pub type jvmtiEventCompiledMethodUnload = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		method: jmethodID,
		code_addr: *const ::std::os::raw::c_void,
	),
>;
pub type jvmtiEventDataDumpRequest =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;
pub type jvmtiEventDynamicCodeGenerated = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		name: *const ::std::os::raw::c_char,
		address: *const ::std::os::raw::c_void,
		length: jint,
	),
>;
pub type jvmtiEventException = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
		exception: jobject,
		catch_method: jmethodID,
		catch_location: jlocation,
	),
>;
pub type jvmtiEventExceptionCatch = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
		exception: jobject,
	),
>;
pub type jvmtiEventFieldAccess = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
		field_klass: jclass,
		object: jobject,
		field: jfieldID,
	),
>;
pub type jvmtiEventFieldModification = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
		field_klass: jclass,
		object: jobject,
		field: jfieldID,
		signature_type: ::std::os::raw::c_char,
		new_value: jvalue,
	),
>;
pub type jvmtiEventFramePop = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		was_popped_by_exception: jboolean,
	),
>;
pub type jvmtiEventGarbageCollectionFinish =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;
pub type jvmtiEventGarbageCollectionStart =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;
pub type jvmtiEventMethodEntry = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
	),
>;
pub type jvmtiEventMethodExit = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		was_popped_by_exception: jboolean,
		return_value: jvalue,
	),
>;
pub type jvmtiEventMonitorContendedEnter = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		object: jobject,
	),
>;
pub type jvmtiEventMonitorContendedEntered = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		object: jobject,
	),
>;
pub type jvmtiEventMonitorWait = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		object: jobject,
		timeout: jlong,
	),
>;
pub type jvmtiEventMonitorWaited = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		object: jobject,
		timed_out: jboolean,
	),
>;
pub type jvmtiEventNativeMethodBind = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		address: *mut ::std::os::raw::c_void,
		new_address_ptr: *mut *mut ::std::os::raw::c_void,
	),
>;
pub type jvmtiEventObjectFree =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, tag: jlong)>;
pub type jvmtiEventResourceExhausted = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		flags: jint,
		reserved: *const ::std::os::raw::c_void,
		description: *const ::std::os::raw::c_char,
	),
>;
pub type jvmtiEventSingleStep = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		method: jmethodID,
		location: jlocation,
	),
>;
pub type jvmtiEventThreadEnd = ::std::option::Option<
	unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;
pub type jvmtiEventThreadStart = ::std::option::Option<
	unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;
pub type jvmtiEventVMDeath =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;
pub type jvmtiEventVMInit = ::std::option::Option<
	unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;
pub type jvmtiEventVMObjectAlloc = ::std::option::Option<
	unsafe extern "C" fn(
		jvmti_env: *mut jvmtiEnv,
		jni_env: *mut JNIEnv,
		thread: jthread,
		object: jobject,
		object_klass: jclass,
		size: jlong,
	),
>;
pub type jvmtiEventVMStart =
::std::option::Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiEventCallbacks {
	pub VMInit: jvmtiEventVMInit,
	pub VMDeath: jvmtiEventVMDeath,
	pub ThreadStart: jvmtiEventThreadStart,
	pub ThreadEnd: jvmtiEventThreadEnd,
	pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
	pub ClassLoad: jvmtiEventClassLoad,
	pub ClassPrepare: jvmtiEventClassPrepare,
	pub VMStart: jvmtiEventVMStart,
	pub Exception: jvmtiEventException,
	pub ExceptionCatch: jvmtiEventExceptionCatch,
	pub SingleStep: jvmtiEventSingleStep,
	pub FramePop: jvmtiEventFramePop,
	pub Breakpoint: jvmtiEventBreakpoint,
	pub FieldAccess: jvmtiEventFieldAccess,
	pub FieldModification: jvmtiEventFieldModification,
	pub MethodEntry: jvmtiEventMethodEntry,
	pub MethodExit: jvmtiEventMethodExit,
	pub NativeMethodBind: jvmtiEventNativeMethodBind,
	pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
	pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
	pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
	pub DataDumpRequest: jvmtiEventDataDumpRequest,
	pub reserved72: jvmtiEventReserved,
	pub MonitorWait: jvmtiEventMonitorWait,
	pub MonitorWaited: jvmtiEventMonitorWaited,
	pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
	pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
	pub reserved77: jvmtiEventReserved,
	pub reserved78: jvmtiEventReserved,
	pub reserved79: jvmtiEventReserved,
	pub ResourceExhausted: jvmtiEventResourceExhausted,
	pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
	pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
	pub ObjectFree: jvmtiEventObjectFree,
	pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiInterface_1_ {
	pub reserved1: *mut ::std::os::raw::c_void,
	pub SetEventNotificationMode: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			mode: jvmtiEventMode,
			event_type: jvmtiEvent,
			event_thread: jthread,
			...
		) -> jvmtiError,
	>,
	pub reserved3: *mut ::std::os::raw::c_void,
	pub GetAllThreads: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			threads_count_ptr: *mut jint,
			threads_ptr: *mut *mut jthread,
		) -> jvmtiError,
	>,
	pub SuspendThread: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	>,
	pub ResumeThread: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	>,
	pub StopThread: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError,
	>,
	pub InterruptThread: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	>,
	pub GetThreadInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			info_ptr: *mut jvmtiThreadInfo,
		) -> jvmtiError,
	>,
	pub GetOwnedMonitorInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			owned_monitor_count_ptr: *mut jint,
			owned_monitors_ptr: *mut *mut jobject,
		) -> jvmtiError,
	>,
	pub GetCurrentContendedMonitor: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			monitor_ptr: *mut jobject,
		) -> jvmtiError,
	>,
	pub RunAgentThread: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			proc_: jvmtiStartFunction,
			arg: *const ::std::os::raw::c_void,
			priority: jint,
		) -> jvmtiError,
	>,
	pub GetTopThreadGroups: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			group_count_ptr: *mut jint,
			groups_ptr: *mut *mut jthreadGroup,
		) -> jvmtiError,
	>,
	pub GetThreadGroupInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			group: jthreadGroup,
			info_ptr: *mut jvmtiThreadGroupInfo,
		) -> jvmtiError,
	>,
	pub GetThreadGroupChildren: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			group: jthreadGroup,
			thread_count_ptr: *mut jint,
			threads_ptr: *mut *mut jthread,
			group_count_ptr: *mut jint,
			groups_ptr: *mut *mut jthreadGroup,
		) -> jvmtiError,
	>,
	pub GetFrameCount: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			count_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetThreadState: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			thread_state_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetCurrentThread: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError,
	>,
	pub GetFrameLocation: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			method_ptr: *mut jmethodID,
			location_ptr: *mut jlocation,
		) -> jvmtiError,
	>,
	pub NotifyFramePop: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
	>,
	pub GetLocalObject: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value_ptr: *mut jobject,
		) -> jvmtiError,
	>,
	pub GetLocalInt: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetLocalLong: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value_ptr: *mut jlong,
		) -> jvmtiError,
	>,
	pub GetLocalFloat: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value_ptr: *mut jfloat,
		) -> jvmtiError,
	>,
	pub GetLocalDouble: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value_ptr: *mut jdouble,
		) -> jvmtiError,
	>,
	pub SetLocalObject: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value: jobject,
		) -> jvmtiError,
	>,
	pub SetLocalInt: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value: jint,
		) -> jvmtiError,
	>,
	pub SetLocalLong: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value: jlong,
		) -> jvmtiError,
	>,
	pub SetLocalFloat: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value: jfloat,
		) -> jvmtiError,
	>,
	pub SetLocalDouble: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			slot: jint,
			value: jdouble,
		) -> jvmtiError,
	>,
	pub CreateRawMonitor: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			name: *const ::std::os::raw::c_char,
			monitor_ptr: *mut jrawMonitorID,
		) -> jvmtiError,
	>,
	pub DestroyRawMonitor: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	>,
	pub RawMonitorEnter: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	>,
	pub RawMonitorExit: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	>,
	pub RawMonitorWait: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			monitor: jrawMonitorID,
			millis: jlong,
		) -> jvmtiError,
	>,
	pub RawMonitorNotify: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	>,
	pub RawMonitorNotifyAll: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
	>,
	pub SetBreakpoint: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			location: jlocation,
		) -> jvmtiError,
	>,
	pub ClearBreakpoint: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			location: jlocation,
		) -> jvmtiError,
	>,
	pub reserved40: *mut ::std::os::raw::c_void,
	pub SetFieldAccessWatch: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	>,
	pub ClearFieldAccessWatch: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	>,
	pub SetFieldModificationWatch: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	>,
	pub ClearFieldModificationWatch: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
	>,
	pub IsModifiableClass: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			is_modifiable_class_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub Allocate: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			size: jlong,
			mem_ptr: *mut *mut ::std::os::raw::c_uchar,
		) -> jvmtiError,
	>,
	pub Deallocate: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, mem: *mut ::std::os::raw::c_uchar) -> jvmtiError,
	>,
	pub GetClassSignature: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			signature_ptr: *mut *mut ::std::os::raw::c_char,
			generic_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetClassStatus: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			status_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetSourceFileName: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			source_name_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetClassModifiers: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			modifiers_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetClassMethods: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			method_count_ptr: *mut jint,
			methods_ptr: *mut *mut jmethodID,
		) -> jvmtiError,
	>,
	pub GetClassFields: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			field_count_ptr: *mut jint,
			fields_ptr: *mut *mut jfieldID,
		) -> jvmtiError,
	>,
	pub GetImplementedInterfaces: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			interface_count_ptr: *mut jint,
			interfaces_ptr: *mut *mut jclass,
		) -> jvmtiError,
	>,
	pub IsInterface: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			is_interface_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub IsArrayClass: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			is_array_class_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub GetClassLoader: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			classloader_ptr: *mut jobject,
		) -> jvmtiError,
	>,
	pub GetObjectHashCode: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object: jobject,
			hash_code_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetObjectMonitorUsage: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object: jobject,
			info_ptr: *mut jvmtiMonitorUsage,
		) -> jvmtiError,
	>,
	pub GetFieldName: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			field: jfieldID,
			name_ptr: *mut *mut ::std::os::raw::c_char,
			signature_ptr: *mut *mut ::std::os::raw::c_char,
			generic_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetFieldDeclaringClass: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			field: jfieldID,
			declaring_class_ptr: *mut jclass,
		) -> jvmtiError,
	>,
	pub GetFieldModifiers: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			field: jfieldID,
			modifiers_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub IsFieldSynthetic: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			field: jfieldID,
			is_synthetic_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub GetMethodName: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			name_ptr: *mut *mut ::std::os::raw::c_char,
			signature_ptr: *mut *mut ::std::os::raw::c_char,
			generic_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetMethodDeclaringClass: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			declaring_class_ptr: *mut jclass,
		) -> jvmtiError,
	>,
	pub GetMethodModifiers: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			modifiers_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub reserved67: *mut ::std::os::raw::c_void,
	pub GetMaxLocals: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			max_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetArgumentsSize: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			size_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetLineNumberTable: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			entry_count_ptr: *mut jint,
			table_ptr: *mut *mut jvmtiLineNumberEntry,
		) -> jvmtiError,
	>,
	pub GetMethodLocation: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			start_location_ptr: *mut jlocation,
			end_location_ptr: *mut jlocation,
		) -> jvmtiError,
	>,
	pub GetLocalVariableTable: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			entry_count_ptr: *mut jint,
			table_ptr: *mut *mut jvmtiLocalVariableEntry,
		) -> jvmtiError,
	>,
	pub SetNativeMethodPrefix: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			prefix: *const ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub SetNativeMethodPrefixes: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			prefix_count: jint,
			prefixes: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetBytecodes: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			bytecode_count_ptr: *mut jint,
			bytecodes_ptr: *mut *mut ::std::os::raw::c_uchar,
		) -> jvmtiError,
	>,
	pub IsMethodNative: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			is_native_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub IsMethodSynthetic: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			is_synthetic_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub GetLoadedClasses: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			class_count_ptr: *mut jint,
			classes_ptr: *mut *mut jclass,
		) -> jvmtiError,
	>,
	pub GetClassLoaderClasses: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			initiating_loader: jobject,
			class_count_ptr: *mut jint,
			classes_ptr: *mut *mut jclass,
		) -> jvmtiError,
	>,
	pub PopFrame: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	>,
	pub ForceEarlyReturnObject: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError,
	>,
	pub ForceEarlyReturnInt: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
	>,
	pub ForceEarlyReturnLong: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
	>,
	pub ForceEarlyReturnFloat: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
	>,
	pub ForceEarlyReturnDouble: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError,
	>,
	pub ForceEarlyReturnVoid: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
	>,
	pub RedefineClasses: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			class_count: jint,
			class_definitions: *const jvmtiClassDefinition,
		) -> jvmtiError,
	>,
	pub GetVersionNumber: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError,
	>,
	pub GetCapabilities: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			capabilities_ptr: *mut jvmtiCapabilities,
		) -> jvmtiError,
	>,
	pub GetSourceDebugExtension: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			source_debug_extension_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub IsMethodObsolete: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			method: jmethodID,
			is_obsolete_ptr: *mut jboolean,
		) -> jvmtiError,
	>,
	pub SuspendThreadList: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			request_count: jint,
			request_list: *const jthread,
			results: *mut jvmtiError,
		) -> jvmtiError,
	>,
	pub ResumeThreadList: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			request_count: jint,
			request_list: *const jthread,
			results: *mut jvmtiError,
		) -> jvmtiError,
	>,
	pub reserved94: *mut ::std::os::raw::c_void,
	pub reserved95: *mut ::std::os::raw::c_void,
	pub reserved96: *mut ::std::os::raw::c_void,
	pub reserved97: *mut ::std::os::raw::c_void,
	pub reserved98: *mut ::std::os::raw::c_void,
	pub reserved99: *mut ::std::os::raw::c_void,
	pub GetAllStackTraces: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			max_frame_count: jint,
			stack_info_ptr: *mut *mut jvmtiStackInfo,
			thread_count_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetThreadListStackTraces: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread_count: jint,
			thread_list: *const jthread,
			max_frame_count: jint,
			stack_info_ptr: *mut *mut jvmtiStackInfo,
		) -> jvmtiError,
	>,
	pub GetThreadLocalStorage: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			data_ptr: *mut *mut ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub SetThreadLocalStorage: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub GetStackTrace: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			start_depth: jint,
			max_frame_count: jint,
			frame_buffer: *mut jvmtiFrameInfo,
			count_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub reserved105: *mut ::std::os::raw::c_void,
	pub GetTag: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object: jobject,
			tag_ptr: *mut jlong,
		) -> jvmtiError,
	>,
	pub SetTag: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError,
	>,
	pub ForceGarbageCollection:
	::std::option::Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
	pub IterateOverObjectsReachableFromObject: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object: jobject,
			object_reference_callback: jvmtiObjectReferenceCallback,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub IterateOverReachableObjects: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			heap_root_callback: jvmtiHeapRootCallback,
			stack_ref_callback: jvmtiStackReferenceCallback,
			object_ref_callback: jvmtiObjectReferenceCallback,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub IterateOverHeap: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object_filter: jvmtiHeapObjectFilter,
			heap_object_callback: jvmtiHeapObjectCallback,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub IterateOverInstancesOfClass: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			object_filter: jvmtiHeapObjectFilter,
			heap_object_callback: jvmtiHeapObjectCallback,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub reserved113: *mut ::std::os::raw::c_void,
	pub GetObjectsWithTags: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			tag_count: jint,
			tags: *const jlong,
			count_ptr: *mut jint,
			object_result_ptr: *mut *mut jobject,
			tag_result_ptr: *mut *mut jlong,
		) -> jvmtiError,
	>,
	pub FollowReferences: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			heap_filter: jint,
			klass: jclass,
			initial_object: jobject,
			callbacks: *const jvmtiHeapCallbacks,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub IterateThroughHeap: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			heap_filter: jint,
			klass: jclass,
			callbacks: *const jvmtiHeapCallbacks,
			user_data: *const ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub reserved117: *mut ::std::os::raw::c_void,
	pub reserved118: *mut ::std::os::raw::c_void,
	pub reserved119: *mut ::std::os::raw::c_void,
	pub SetJNIFunctionTable: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			function_table: *const jniNativeInterface,
		) -> jvmtiError,
	>,
	pub GetJNIFunctionTable: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			function_table: *mut *mut jniNativeInterface,
		) -> jvmtiError,
	>,
	pub SetEventCallbacks: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			callbacks: *const jvmtiEventCallbacks,
			size_of_callbacks: jint,
		) -> jvmtiError,
	>,
	pub GenerateEvents: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError,
	>,
	pub GetExtensionFunctions: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			extension_count_ptr: *mut jint,
			extensions: *mut *mut jvmtiExtensionFunctionInfo,
		) -> jvmtiError,
	>,
	pub GetExtensionEvents: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			extension_count_ptr: *mut jint,
			extensions: *mut *mut jvmtiExtensionEventInfo,
		) -> jvmtiError,
	>,
	pub SetExtensionEventCallback: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			extension_event_index: jint,
			callback: jvmtiExtensionEvent,
		) -> jvmtiError,
	>,
	pub DisposeEnvironment:
	::std::option::Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
	pub GetErrorName: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			error: jvmtiError,
			name_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetJLocationFormat: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			format_ptr: *mut jvmtiJlocationFormat,
		) -> jvmtiError,
	>,
	pub GetSystemProperties: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			count_ptr: *mut jint,
			property_ptr: *mut *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetSystemProperty: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			property: *const ::std::os::raw::c_char,
			value_ptr: *mut *mut ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub SetSystemProperty: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			property: *const ::std::os::raw::c_char,
			value: *const ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub GetPhase: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError,
	>,
	pub GetCurrentThreadCpuTimerInfo: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	>,
	pub GetCurrentThreadCpuTime: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
	>,
	pub GetThreadCpuTimerInfo: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	>,
	pub GetThreadCpuTime: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			nanos_ptr: *mut jlong,
		) -> jvmtiError,
	>,
	pub GetTimerInfo: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
	>,
	pub GetTime: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
	>,
	pub GetPotentialCapabilities: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			capabilities_ptr: *mut jvmtiCapabilities,
		) -> jvmtiError,
	>,
	pub reserved141: *mut ::std::os::raw::c_void,
	pub AddCapabilities: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			capabilities_ptr: *const jvmtiCapabilities,
		) -> jvmtiError,
	>,
	pub RelinquishCapabilities: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			capabilities_ptr: *const jvmtiCapabilities,
		) -> jvmtiError,
	>,
	pub GetAvailableProcessors: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
	>,
	pub GetClassVersionNumbers: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			minor_version_ptr: *mut jint,
			major_version_ptr: *mut jint,
		) -> jvmtiError,
	>,
	pub GetConstantPool: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			klass: jclass,
			constant_pool_count_ptr: *mut jint,
			constant_pool_byte_count_ptr: *mut jint,
			constant_pool_bytes_ptr: *mut *mut ::std::os::raw::c_uchar,
		) -> jvmtiError,
	>,
	pub GetEnvironmentLocalStorage: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			data_ptr: *mut *mut ::std::os::raw::c_void,
		) -> jvmtiError,
	>,
	pub SetEnvironmentLocalStorage: ::std::option::Option<
		unsafe extern "C" fn(env: *mut jvmtiEnv, data: *const ::std::os::raw::c_void) -> jvmtiError,
	>,
	pub AddToBootstrapClassLoaderSearch: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			segment: *const ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub SetVerboseFlag: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			flag: jvmtiVerboseFlag,
			value: jboolean,
		) -> jvmtiError,
	>,
	pub AddToSystemClassLoaderSearch: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			segment: *const ::std::os::raw::c_char,
		) -> jvmtiError,
	>,
	pub RetransformClasses: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			class_count: jint,
			classes: *const jclass,
		) -> jvmtiError,
	>,
	pub GetOwnedMonitorStackDepthInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			monitor_info_count_ptr: *mut jint,
			monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
		) -> jvmtiError,
	>,
	pub GetObjectSize: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			object: jobject,
			size_ptr: *mut jlong,
		) -> jvmtiError,
	>,
	pub GetLocalInstance: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut jvmtiEnv,
			thread: jthread,
			depth: jint,
			value_ptr: *mut jobject,
		) -> jvmtiError,
	>,
}

pub type jvmtiInterface_1 = jvmtiInterface_1_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiEnv {
	pub functions: *const jvmtiInterface_1_,
}

pub type __builtin_va_list = [__va_list_tag; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
	pub gp_offset: ::std::os::raw::c_uint,
	pub fp_offset: ::std::os::raw::c_uint,
	pub overflow_arg_area: *mut ::std::os::raw::c_void,
	pub reg_save_area: *mut ::std::os::raw::c_void,
}
