#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use jni::sys::{jint, jobject, jlong, jmethodID, jclass, jboolean, JNIEnv, jvalue, jfloat, jdouble, jstring, jobjectArray, jintArray, jbyte, jsize, jbyteArray};
use crate::bitfield::__BindgenBitfieldUnit;

pub type size_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
	_unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
	pub _flags: ::std::os::raw::c_int,
	pub _IO_read_ptr: *mut ::std::os::raw::c_char,
	pub _IO_read_end: *mut ::std::os::raw::c_char,
	pub _IO_read_base: *mut ::std::os::raw::c_char,
	pub _IO_write_base: *mut ::std::os::raw::c_char,
	pub _IO_write_ptr: *mut ::std::os::raw::c_char,
	pub _IO_write_end: *mut ::std::os::raw::c_char,
	pub _IO_buf_base: *mut ::std::os::raw::c_char,
	pub _IO_buf_end: *mut ::std::os::raw::c_char,
	pub _IO_save_base: *mut ::std::os::raw::c_char,
	pub _IO_backup_base: *mut ::std::os::raw::c_char,
	pub _IO_save_end: *mut ::std::os::raw::c_char,
	pub _markers: *mut _IO_marker,
	pub _chain: *mut _IO_FILE,
	pub _fileno: ::std::os::raw::c_int,
	pub _flags2: ::std::os::raw::c_int,
	pub _old_offset: __off_t,
	pub _cur_column: ::std::os::raw::c_ushort,
	pub _vtable_offset: ::std::os::raw::c_schar,
	pub _shortbuf: [::std::os::raw::c_char; 1usize],
	pub _lock: *mut _IO_lock_t,
	pub _offset: __off64_t,
	pub _codecvt: *mut _IO_codecvt,
	pub _wide_data: *mut _IO_wide_data,
	pub _freeres_list: *mut _IO_FILE,
	pub _freeres_buf: *mut ::std::os::raw::c_void,
	pub __pad5: size_t,
	pub _mode: ::std::os::raw::c_int,
	pub _unused2: [::std::os::raw::c_char; 20usize],
}
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;

pub const JVM_INTERFACE_VERSION: u32 = 4;
pub const JVM_CALLER_DEPTH: i32 = -1;
pub const JVM_TRACING_DTRACE_VERSION: u32 = 1;
pub const JVM_CLASSFILE_MAJOR_VERSION: u32 = 52;
pub const JVM_CLASSFILE_MINOR_VERSION: u32 = 0;
pub const JVM_IO_ERR: i32 = -1;
pub const JVM_IO_INTR: i32 = -2;
pub const JVM_EEXIST: i32 = -100;

extern "C" {
	pub fn JVM_GetInterfaceVersion() -> jint;
}

extern "C" {
	#[doc = "PART 1: Functions for Native Libraries"]
	pub fn JVM_IHashCode(env: *mut JNIEnv, obj: jobject) -> jint;
}

extern "C" {
	pub fn JVM_MonitorWait(env: *mut JNIEnv, obj: jobject, ms: jlong);
}

extern "C" {
	pub fn JVM_MonitorNotify(env: *mut JNIEnv, obj: jobject);
}

extern "C" {
	pub fn JVM_MonitorNotifyAll(env: *mut JNIEnv, obj: jobject);
}

extern "C" {
	pub fn JVM_Clone(env: *mut JNIEnv, obj: jobject) -> jobject;
}

extern "C" {
	pub fn JVM_InternString(env: *mut JNIEnv, str_: jstring) -> jstring;
}

extern "C" {
	pub fn JVM_CurrentTimeMillis(env: *mut JNIEnv, ignored: jclass) -> jlong;
}

extern "C" {
	pub fn JVM_NanoTime(env: *mut JNIEnv, ignored: jclass) -> jlong;
}

extern "C" {
	pub fn JVM_ArrayCopy(
		env: *mut JNIEnv,
		ignored: jclass,
		src: jobject,
		src_pos: jint,
		dst: jobject,
		dst_pos: jint,
		length: jint,
	);
}

extern "C" {
	pub fn JVM_InitProperties(env: *mut JNIEnv, p: jobject) -> jobject;
}

extern "C" {
	pub fn JVM_OnExit(func: ::std::option::Option<unsafe extern "C" fn()>);
}

extern "C" {
	pub fn JVM_Exit(code: jint);
}

extern "C" {
	pub fn JVM_Halt(code: jint);
}

extern "C" {
	pub fn JVM_GC();
}

extern "C" {
	pub fn JVM_MaxObjectInspectionAge() -> jlong;
}

extern "C" {
	pub fn JVM_TraceInstructions(on: jboolean);
}

extern "C" {
	pub fn JVM_TraceMethodCalls(on: jboolean);
}

extern "C" {
	pub fn JVM_TotalMemory() -> jlong;
}

extern "C" {
	pub fn JVM_FreeMemory() -> jlong;
}

extern "C" {
	pub fn JVM_MaxMemory() -> jlong;
}

extern "C" {
	pub fn JVM_ActiveProcessorCount() -> jint;
}

extern "C" {
	pub fn JVM_LoadLibrary(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}

extern "C" {
	pub fn JVM_UnloadLibrary(handle: *mut ::std::os::raw::c_void);
}

extern "C" {
	pub fn JVM_FindLibraryEntry(
		handle: *mut ::std::os::raw::c_void,
		name: *const ::std::os::raw::c_char,
	) -> *mut ::std::os::raw::c_void;
}

extern "C" {
	pub fn JVM_IsSupportedJNIVersion(version: jint) -> jboolean;
}

extern "C" {
	pub fn JVM_IsNaN(d: jdouble) -> jboolean;
}

extern "C" {
	pub fn JVM_FillInStackTrace(env: *mut JNIEnv, throwable: jobject);
}

extern "C" {
	pub fn JVM_GetStackTraceDepth(env: *mut JNIEnv, throwable: jobject) -> jint;
}

extern "C" {
	pub fn JVM_GetStackTraceElement(env: *mut JNIEnv, throwable: jobject, index: jint) -> jobject;
}

extern "C" {
	pub fn JVM_InitializeCompiler(env: *mut JNIEnv, compCls: jclass);
}

extern "C" {
	pub fn JVM_IsSilentCompiler(env: *mut JNIEnv, compCls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_CompileClass(env: *mut JNIEnv, compCls: jclass, cls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_CompileClasses(env: *mut JNIEnv, cls: jclass, jname: jstring) -> jboolean;
}

extern "C" {
	pub fn JVM_CompilerCommand(env: *mut JNIEnv, compCls: jclass, arg: jobject) -> jobject;
}

extern "C" {
	pub fn JVM_EnableCompiler(env: *mut JNIEnv, compCls: jclass);
}

extern "C" {
	pub fn JVM_DisableCompiler(env: *mut JNIEnv, compCls: jclass);
}

extern "C" {
	pub fn JVM_StartThread(env: *mut JNIEnv, thread: jobject);
}

extern "C" {
	pub fn JVM_StopThread(env: *mut JNIEnv, thread: jobject, exception: jobject);
}

extern "C" {
	pub fn JVM_IsThreadAlive(env: *mut JNIEnv, thread: jobject) -> jboolean;
}

extern "C" {
	pub fn JVM_SuspendThread(env: *mut JNIEnv, thread: jobject);
}

extern "C" {
	pub fn JVM_ResumeThread(env: *mut JNIEnv, thread: jobject);
}

extern "C" {
	pub fn JVM_SetThreadPriority(env: *mut JNIEnv, thread: jobject, prio: jint);
}

extern "C" {
	pub fn JVM_Yield(env: *mut JNIEnv, threadClass: jclass);
}

extern "C" {
	pub fn JVM_Sleep(env: *mut JNIEnv, threadClass: jclass, millis: jlong);
}

extern "C" {
	pub fn JVM_CurrentThread(env: *mut JNIEnv, threadClass: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_CountStackFrames(env: *mut JNIEnv, thread: jobject) -> jint;
}

extern "C" {
	pub fn JVM_Interrupt(env: *mut JNIEnv, thread: jobject);
}

extern "C" {
	pub fn JVM_IsInterrupted(
		env: *mut JNIEnv,
		thread: jobject,
		clearInterrupted: jboolean,
	) -> jboolean;
}

extern "C" {
	pub fn JVM_HoldsLock(env: *mut JNIEnv, threadClass: jclass, obj: jobject) -> jboolean;
}

extern "C" {
	pub fn JVM_DumpAllStacks(env: *mut JNIEnv, unused: jclass);
}

extern "C" {
	pub fn JVM_GetAllThreads(env: *mut JNIEnv, dummy: jclass) -> jobjectArray;
}

extern "C" {
	pub fn JVM_SetNativeThreadName(env: *mut JNIEnv, jthread: jobject, name: jstring);
}

extern "C" {
	pub fn JVM_DumpThreads(
		env: *mut JNIEnv,
		threadClass: jclass,
		threads: jobjectArray,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_CurrentLoadedClass(env: *mut JNIEnv) -> jclass;
}

extern "C" {
	pub fn JVM_CurrentClassLoader(env: *mut JNIEnv) -> jobject;
}

extern "C" {
	pub fn JVM_GetClassContext(env: *mut JNIEnv) -> jobjectArray;
}

extern "C" {
	pub fn JVM_ClassDepth(env: *mut JNIEnv, name: jstring) -> jint;
}

extern "C" {
	pub fn JVM_ClassLoaderDepth(env: *mut JNIEnv) -> jint;
}

extern "C" {
	pub fn JVM_GetSystemPackage(env: *mut JNIEnv, name: jstring) -> jstring;
}

extern "C" {
	pub fn JVM_GetSystemPackages(env: *mut JNIEnv) -> jobjectArray;
}

extern "C" {
	pub fn JVM_AllocateNewObject(
		env: *mut JNIEnv,
		obj: jobject,
		currClass: jclass,
		initClass: jclass,
	) -> jobject;
}

extern "C" {
	pub fn JVM_AllocateNewArray(
		env: *mut JNIEnv,
		obj: jobject,
		currClass: jclass,
		length: jint,
	) -> jobject;
}

extern "C" {
	pub fn JVM_LatestUserDefinedLoader(env: *mut JNIEnv) -> jobject;
}

extern "C" {
	pub fn JVM_LoadClass0(
		env: *mut JNIEnv,
		obj: jobject,
		currClass: jclass,
		currClassName: jstring,
	) -> jclass;
}

extern "C" {
	pub fn JVM_GetArrayLength(env: *mut JNIEnv, arr: jobject) -> jint;
}

extern "C" {
	pub fn JVM_GetArrayElement(env: *mut JNIEnv, arr: jobject, index: jint) -> jobject;
}

extern "C" {
	pub fn JVM_GetPrimitiveArrayElement(
		env: *mut JNIEnv,
		arr: jobject,
		index: jint,
		wCode: jint,
	) -> jvalue;
}

extern "C" {
	pub fn JVM_SetArrayElement(env: *mut JNIEnv, arr: jobject, index: jint, val: jobject);
}

extern "C" {
	pub fn JVM_SetPrimitiveArrayElement(
		env: *mut JNIEnv,
		arr: jobject,
		index: jint,
		v: jvalue,
		vCode: ::std::os::raw::c_uchar,
	);
}

extern "C" {
	pub fn JVM_NewArray(env: *mut JNIEnv, eltClass: jclass, length: jint) -> jobject;
}

extern "C" {
	pub fn JVM_NewMultiArray(env: *mut JNIEnv, eltClass: jclass, dim: jintArray) -> jobject;
}

extern "C" {
	pub fn JVM_GetCallerClass(env: *mut JNIEnv, depth: ::std::os::raw::c_int) -> jclass;
}

extern "C" {
	pub fn JVM_FindPrimitiveClass(env: *mut JNIEnv, utf: *const ::std::os::raw::c_char) -> jclass;
}

extern "C" {
	pub fn JVM_ResolveClass(env: *mut JNIEnv, cls: jclass);
}

extern "C" {
	pub fn JVM_FindClassFromBootLoader(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
	) -> jclass;
}

extern "C" {
	pub fn JVM_FindClassFromCaller(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		init: jboolean,
		loader: jobject,
		caller: jclass,
	) -> jclass;
}

extern "C" {
	pub fn JVM_FindClassFromClassLoader(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		init: jboolean,
		loader: jobject,
		throwError: jboolean,
	) -> jclass;
}

extern "C" {
	pub fn JVM_FindClassFromClass(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		init: jboolean,
		from: jclass,
	) -> jclass;
}

extern "C" {
	pub fn JVM_FindLoadedClass(env: *mut JNIEnv, loader: jobject, name: jstring) -> jclass;
}

extern "C" {
	pub fn JVM_DefineClass(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		loader: jobject,
		buf: *const jbyte,
		len: jsize,
		pd: jobject,
	) -> jclass;
}

extern "C" {
	pub fn JVM_DefineClassWithSource(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		loader: jobject,
		buf: *const jbyte,
		len: jsize,
		pd: jobject,
		source: *const ::std::os::raw::c_char,
	) -> jclass;
}

extern "C" {
	pub fn JVM_GetClassName(env: *mut JNIEnv, cls: jclass) -> jstring;
}

extern "C" {
	pub fn JVM_GetClassInterfaces(env: *mut JNIEnv, cls: jclass) -> jobjectArray;
}

extern "C" {
	pub fn JVM_IsInterface(env: *mut JNIEnv, cls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_GetClassSigners(env: *mut JNIEnv, cls: jclass) -> jobjectArray;
}

extern "C" {
	pub fn JVM_SetClassSigners(env: *mut JNIEnv, cls: jclass, signers: jobjectArray);
}

extern "C" {
	pub fn JVM_GetProtectionDomain(env: *mut JNIEnv, cls: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_IsArrayClass(env: *mut JNIEnv, cls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_IsPrimitiveClass(env: *mut JNIEnv, cls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_GetComponentType(env: *mut JNIEnv, cls: jclass) -> jclass;
}

extern "C" {
	pub fn JVM_GetClassModifiers(env: *mut JNIEnv, cls: jclass) -> jint;
}

extern "C" {
	pub fn JVM_GetDeclaredClasses(env: *mut JNIEnv, ofClass: jclass) -> jobjectArray;
}

extern "C" {
	pub fn JVM_GetDeclaringClass(env: *mut JNIEnv, ofClass: jclass) -> jclass;
}

extern "C" {
	pub fn JVM_GetClassSignature(env: *mut JNIEnv, cls: jclass) -> jstring;
}

extern "C" {
	pub fn JVM_GetClassAnnotations(env: *mut JNIEnv, cls: jclass) -> jbyteArray;
}

extern "C" {
	pub fn JVM_GetClassTypeAnnotations(env: *mut JNIEnv, cls: jclass) -> jbyteArray;
}

extern "C" {
	pub fn JVM_GetFieldTypeAnnotations(env: *mut JNIEnv, field: jobject) -> jbyteArray;
}

extern "C" {
	pub fn JVM_GetMethodTypeAnnotations(env: *mut JNIEnv, method: jobject) -> jbyteArray;
}

extern "C" {
	pub fn JVM_GetClassDeclaredMethods(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_GetClassDeclaredFields(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_GetClassDeclaredConstructors(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_GetClassAccessFlags(env: *mut JNIEnv, cls: jclass) -> jint;
}

extern "C" {
	pub fn JVM_InvokeMethod(
		env: *mut JNIEnv,
		method: jobject,
		obj: jobject,
		args0: jobjectArray,
	) -> jobject;
}

extern "C" {
	pub fn JVM_NewInstanceFromConstructor(
		env: *mut JNIEnv,
		c: jobject,
		args0: jobjectArray,
	) -> jobject;
}

extern "C" {
	pub fn JVM_GetClassConstantPool(env: *mut JNIEnv, cls: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_ConstantPoolGetSize(env: *mut JNIEnv, unused: jobject, jcpool: jobject) -> jint;
}

extern "C" {
	pub fn JVM_ConstantPoolGetClassAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jclass;
}

extern "C" {
	pub fn JVM_ConstantPoolGetClassAtIfLoaded(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jclass;
}

extern "C" {
	pub fn JVM_ConstantPoolGetMethodAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject;
}

extern "C" {
	pub fn JVM_ConstantPoolGetMethodAtIfLoaded(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject;
}

extern "C" {
	pub fn JVM_ConstantPoolGetFieldAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject;
}

extern "C" {
	pub fn JVM_ConstantPoolGetFieldAtIfLoaded(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject;
}

extern "C" {
	pub fn JVM_ConstantPoolGetMemberRefInfoAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_ConstantPoolGetIntAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jint;
}

extern "C" {
	pub fn JVM_ConstantPoolGetLongAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jlong;
}

extern "C" {
	pub fn JVM_ConstantPoolGetFloatAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jfloat;
}

extern "C" {
	pub fn JVM_ConstantPoolGetDoubleAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jdouble;
}

extern "C" {
	pub fn JVM_ConstantPoolGetStringAt(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jstring;
}

extern "C" {
	pub fn JVM_ConstantPoolGetUTF8At(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jstring;
}

extern "C" {
	pub fn JVM_GetMethodParameters(env: *mut JNIEnv, method: jobject) -> jobjectArray;
}

extern "C" {
	pub fn JVM_DoPrivileged(
		env: *mut JNIEnv,
		cls: jclass,
		action: jobject,
		context: jobject,
		wrapException: jboolean,
	) -> jobject;
}

extern "C" {
	pub fn JVM_GetInheritedAccessControlContext(env: *mut JNIEnv, cls: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_GetStackAccessControlContext(env: *mut JNIEnv, cls: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_RegisterSignal(
		sig: jint,
		handler: *mut ::std::os::raw::c_void,
	) -> *mut ::std::os::raw::c_void;
}

extern "C" {
	pub fn JVM_RaiseSignal(sig: jint) -> jboolean;
}

extern "C" {
	pub fn JVM_FindSignal(name: *const ::std::os::raw::c_char) -> jint;
}

extern "C" {
	pub fn JVM_DesiredAssertionStatus(env: *mut JNIEnv, unused: jclass, cls: jclass) -> jboolean;
}

extern "C" {
	pub fn JVM_AssertionStatusDirectives(env: *mut JNIEnv, unused: jclass) -> jobject;
}

extern "C" {
	pub fn JVM_SupportsCX8() -> jboolean;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceProbe {
	pub method: jmethodID,
	pub function: jstring,
	pub name: jstring,
	pub reserved: [*mut ::std::os::raw::c_void; 4usize],
}

#[doc = " Encapsulates the stability ratings for a DTrace provider field"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceInterfaceAttributes {
	pub nameStability: jint,
	pub dataStability: jint,
	pub dependencyClass: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceProvider {
	pub name: jstring,
	pub probes: *mut JVM_DTraceProbe,
	pub probe_count: jint,
	pub providerAttributes: JVM_DTraceInterfaceAttributes,
	pub moduleAttributes: JVM_DTraceInterfaceAttributes,
	pub functionAttributes: JVM_DTraceInterfaceAttributes,
	pub nameAttributes: JVM_DTraceInterfaceAttributes,
	pub argsAttributes: JVM_DTraceInterfaceAttributes,
	pub reserved: [*mut ::std::os::raw::c_void; 4usize],
}

extern "C" {
	pub fn JVM_DTraceGetVersion(env: *mut JNIEnv) -> jint;
}

extern "C" {
	pub fn JVM_DTraceActivate(
		env: *mut JNIEnv,
		version: jint,
		module_name: jstring,
		providers_count: jint,
		providers: *mut JVM_DTraceProvider,
	) -> jlong;
}

extern "C" {
	pub fn JVM_DTraceIsProbeEnabled(env: *mut JNIEnv, method: jmethodID) -> jboolean;
}

extern "C" {
	pub fn JVM_DTraceDispose(env: *mut JNIEnv, activation_handle: jlong);
}

extern "C" {
	pub fn JVM_DTraceIsSupported(env: *mut JNIEnv) -> jboolean;
}

extern "C" {
	#[doc = "PART 2: Support for the Verifier and Class File Format Checker"]
	pub fn JVM_GetClassNameUTF(env: *mut JNIEnv, cb: jclass) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetClassCPTypes(env: *mut JNIEnv, cb: jclass, types: *mut ::std::os::raw::c_uchar);
}

extern "C" {
	pub fn JVM_GetClassCPEntriesCount(env: *mut JNIEnv, cb: jclass) -> jint;
}

extern "C" {
	pub fn JVM_GetClassFieldsCount(env: *mut JNIEnv, cb: jclass) -> jint;
}

extern "C" {
	pub fn JVM_GetClassMethodsCount(env: *mut JNIEnv, cb: jclass) -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxExceptionIndexes(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		exceptions: *mut ::std::os::raw::c_ushort,
	);
}

extern "C" {
	pub fn JVM_GetMethodIxExceptionsCount(env: *mut JNIEnv, cb: jclass, method_index: jint)
	                                      -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxByteCode(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		code: *mut ::std::os::raw::c_uchar,
	);
}

extern "C" {
	pub fn JVM_GetMethodIxByteCodeLength(env: *mut JNIEnv, cb: jclass, method_index: jint) -> jint;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_ExceptionTableEntryType {
	pub start_pc: jint,
	pub end_pc: jint,
	pub handler_pc: jint,
	pub catchType: jint,
}

extern "C" {
	pub fn JVM_GetMethodIxExceptionTableEntry(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		entry_index: jint,
		entry: *mut JVM_ExceptionTableEntryType,
	);
}

extern "C" {
	pub fn JVM_GetMethodIxExceptionTableLength(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetFieldIxModifiers(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxModifiers(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxLocalsCount(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxArgsSize(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetMethodIxMaxStack(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_IsConstructorIx(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jboolean;
}

extern "C" {
	pub fn JVM_IsVMGeneratedMethodIx(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jboolean;
}

extern "C" {
	pub fn JVM_GetMethodIxNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetMethodIxSignatureUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPFieldNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPMethodNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPMethodSignatureUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPFieldSignatureUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPClassNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPFieldClassNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPMethodClassNameUTF(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_GetCPFieldModifiers(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
		calledClass: jclass,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetCPMethodModifiers(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
		calledClass: jclass,
	) -> jint;
}

extern "C" {
	pub fn JVM_ReleaseUTF(utf: *const ::std::os::raw::c_char);
}

extern "C" {
	pub fn JVM_IsSameClassPackage(env: *mut JNIEnv, class1: jclass, class2: jclass) -> jboolean;
}

pub const JVM_ACC_PUBLIC: ::std::os::raw::c_uint = 1;
pub const JVM_ACC_PRIVATE: ::std::os::raw::c_uint = 2;
pub const JVM_ACC_PROTECTED: ::std::os::raw::c_uint = 4;
pub const JVM_ACC_STATIC: ::std::os::raw::c_uint = 8;
pub const JVM_ACC_FINAL: ::std::os::raw::c_uint = 16;
pub const JVM_ACC_SYNCHRONIZED: ::std::os::raw::c_uint = 32;
pub const JVM_ACC_SUPER: ::std::os::raw::c_uint = 32;
pub const JVM_ACC_VOLATILE: ::std::os::raw::c_uint = 64;
pub const JVM_ACC_BRIDGE: ::std::os::raw::c_uint = 64;
pub const JVM_ACC_TRANSIENT: ::std::os::raw::c_uint = 128;
pub const JVM_ACC_VARARGS: ::std::os::raw::c_uint = 128;
pub const JVM_ACC_NATIVE: ::std::os::raw::c_uint = 256;
pub const JVM_ACC_INTERFACE: ::std::os::raw::c_uint = 512;
pub const JVM_ACC_ABSTRACT: ::std::os::raw::c_uint = 1024;
pub const JVM_ACC_STRICT: ::std::os::raw::c_uint = 2048;
pub const JVM_ACC_SYNTHETIC: ::std::os::raw::c_uint = 4096;
pub const JVM_ACC_ANNOTATION: ::std::os::raw::c_uint = 8192;
pub const JVM_ACC_ENUM: ::std::os::raw::c_uint = 16384;
pub const JVM_T_BOOLEAN: ::std::os::raw::c_uint = 4;
pub const JVM_T_CHAR: ::std::os::raw::c_uint = 5;
pub const JVM_T_FLOAT: ::std::os::raw::c_uint = 6;
pub const JVM_T_DOUBLE: ::std::os::raw::c_uint = 7;
pub const JVM_T_BYTE: ::std::os::raw::c_uint = 8;
pub const JVM_T_SHORT: ::std::os::raw::c_uint = 9;
pub const JVM_T_INT: ::std::os::raw::c_uint = 10;
pub const JVM_T_LONG: ::std::os::raw::c_uint = 11;
pub const JVM_CONSTANT_Utf8: ::std::os::raw::c_uint = 1;
pub const JVM_CONSTANT_Unicode: ::std::os::raw::c_uint = 2;
pub const JVM_CONSTANT_Integer: ::std::os::raw::c_uint = 3;
pub const JVM_CONSTANT_Float: ::std::os::raw::c_uint = 4;
pub const JVM_CONSTANT_Long: ::std::os::raw::c_uint = 5;
pub const JVM_CONSTANT_Double: ::std::os::raw::c_uint = 6;
pub const JVM_CONSTANT_Class: ::std::os::raw::c_uint = 7;
pub const JVM_CONSTANT_String: ::std::os::raw::c_uint = 8;
pub const JVM_CONSTANT_Fieldref: ::std::os::raw::c_uint = 9;
pub const JVM_CONSTANT_Methodref: ::std::os::raw::c_uint = 10;
pub const JVM_CONSTANT_InterfaceMethodref: ::std::os::raw::c_uint = 11;
pub const JVM_CONSTANT_NameAndType: ::std::os::raw::c_uint = 12;
pub const JVM_CONSTANT_MethodHandle: ::std::os::raw::c_uint = 15;
pub const JVM_CONSTANT_MethodType: ::std::os::raw::c_uint = 16;
pub const JVM_CONSTANT_InvokeDynamic: ::std::os::raw::c_uint = 18;
pub const JVM_REF_getField: ::std::os::raw::c_uint = 1;
pub const JVM_REF_getStatic: ::std::os::raw::c_uint = 2;
pub const JVM_REF_putField: ::std::os::raw::c_uint = 3;
pub const JVM_REF_putStatic: ::std::os::raw::c_uint = 4;
pub const JVM_REF_invokeVirtual: ::std::os::raw::c_uint = 5;
pub const JVM_REF_invokeStatic: ::std::os::raw::c_uint = 6;
pub const JVM_REF_invokeSpecial: ::std::os::raw::c_uint = 7;
pub const JVM_REF_newInvokeSpecial: ::std::os::raw::c_uint = 8;
pub const JVM_REF_invokeInterface: ::std::os::raw::c_uint = 9;
pub const JVM_ITEM_Top: ::std::os::raw::c_uint = 0;
pub const JVM_ITEM_Integer: ::std::os::raw::c_uint = 1;
pub const JVM_ITEM_Float: ::std::os::raw::c_uint = 2;
pub const JVM_ITEM_Double: ::std::os::raw::c_uint = 3;
pub const JVM_ITEM_Long: ::std::os::raw::c_uint = 4;
pub const JVM_ITEM_Null: ::std::os::raw::c_uint = 5;
pub const JVM_ITEM_UninitializedThis: ::std::os::raw::c_uint = 6;
pub const JVM_ITEM_Object: ::std::os::raw::c_uint = 7;
pub const JVM_ITEM_Uninitialized: ::std::os::raw::c_uint = 8;
pub const JVM_SIGNATURE_ARRAY: ::std::os::raw::c_uint = 91;
pub const JVM_SIGNATURE_BYTE: ::std::os::raw::c_uint = 66;
pub const JVM_SIGNATURE_CHAR: ::std::os::raw::c_uint = 67;
pub const JVM_SIGNATURE_CLASS: ::std::os::raw::c_uint = 76;
pub const JVM_SIGNATURE_ENDCLASS: ::std::os::raw::c_uint = 59;
pub const JVM_SIGNATURE_ENUM: ::std::os::raw::c_uint = 69;
pub const JVM_SIGNATURE_FLOAT: ::std::os::raw::c_uint = 70;
pub const JVM_SIGNATURE_DOUBLE: ::std::os::raw::c_uint = 68;
pub const JVM_SIGNATURE_FUNC: ::std::os::raw::c_uint = 40;
pub const JVM_SIGNATURE_ENDFUNC: ::std::os::raw::c_uint = 41;
pub const JVM_SIGNATURE_INT: ::std::os::raw::c_uint = 73;
pub const JVM_SIGNATURE_LONG: ::std::os::raw::c_uint = 74;
pub const JVM_SIGNATURE_SHORT: ::std::os::raw::c_uint = 83;
pub const JVM_SIGNATURE_VOID: ::std::os::raw::c_uint = 86;
pub const JVM_SIGNATURE_BOOLEAN: ::std::os::raw::c_uint = 90;
pub const JVM_OPC_nop: ::std::os::raw::c_uint = 0;
pub const JVM_OPC_aconst_null: ::std::os::raw::c_uint = 1;
pub const JVM_OPC_iconst_m1: ::std::os::raw::c_uint = 2;
pub const JVM_OPC_iconst_0: ::std::os::raw::c_uint = 3;
pub const JVM_OPC_iconst_1: ::std::os::raw::c_uint = 4;
pub const JVM_OPC_iconst_2: ::std::os::raw::c_uint = 5;
pub const JVM_OPC_iconst_3: ::std::os::raw::c_uint = 6;
pub const JVM_OPC_iconst_4: ::std::os::raw::c_uint = 7;
pub const JVM_OPC_iconst_5: ::std::os::raw::c_uint = 8;
pub const JVM_OPC_lconst_0: ::std::os::raw::c_uint = 9;
pub const JVM_OPC_lconst_1: ::std::os::raw::c_uint = 10;
pub const JVM_OPC_fconst_0: ::std::os::raw::c_uint = 11;
pub const JVM_OPC_fconst_1: ::std::os::raw::c_uint = 12;
pub const JVM_OPC_fconst_2: ::std::os::raw::c_uint = 13;
pub const JVM_OPC_dconst_0: ::std::os::raw::c_uint = 14;
pub const JVM_OPC_dconst_1: ::std::os::raw::c_uint = 15;
pub const JVM_OPC_bipush: ::std::os::raw::c_uint = 16;
pub const JVM_OPC_sipush: ::std::os::raw::c_uint = 17;
pub const JVM_OPC_ldc: ::std::os::raw::c_uint = 18;
pub const JVM_OPC_ldc_w: ::std::os::raw::c_uint = 19;
pub const JVM_OPC_ldc2_w: ::std::os::raw::c_uint = 20;
pub const JVM_OPC_iload: ::std::os::raw::c_uint = 21;
pub const JVM_OPC_lload: ::std::os::raw::c_uint = 22;
pub const JVM_OPC_fload: ::std::os::raw::c_uint = 23;
pub const JVM_OPC_dload: ::std::os::raw::c_uint = 24;
pub const JVM_OPC_aload: ::std::os::raw::c_uint = 25;
pub const JVM_OPC_iload_0: ::std::os::raw::c_uint = 26;
pub const JVM_OPC_iload_1: ::std::os::raw::c_uint = 27;
pub const JVM_OPC_iload_2: ::std::os::raw::c_uint = 28;
pub const JVM_OPC_iload_3: ::std::os::raw::c_uint = 29;
pub const JVM_OPC_lload_0: ::std::os::raw::c_uint = 30;
pub const JVM_OPC_lload_1: ::std::os::raw::c_uint = 31;
pub const JVM_OPC_lload_2: ::std::os::raw::c_uint = 32;
pub const JVM_OPC_lload_3: ::std::os::raw::c_uint = 33;
pub const JVM_OPC_fload_0: ::std::os::raw::c_uint = 34;
pub const JVM_OPC_fload_1: ::std::os::raw::c_uint = 35;
pub const JVM_OPC_fload_2: ::std::os::raw::c_uint = 36;
pub const JVM_OPC_fload_3: ::std::os::raw::c_uint = 37;
pub const JVM_OPC_dload_0: ::std::os::raw::c_uint = 38;
pub const JVM_OPC_dload_1: ::std::os::raw::c_uint = 39;
pub const JVM_OPC_dload_2: ::std::os::raw::c_uint = 40;
pub const JVM_OPC_dload_3: ::std::os::raw::c_uint = 41;
pub const JVM_OPC_aload_0: ::std::os::raw::c_uint = 42;
pub const JVM_OPC_aload_1: ::std::os::raw::c_uint = 43;
pub const JVM_OPC_aload_2: ::std::os::raw::c_uint = 44;
pub const JVM_OPC_aload_3: ::std::os::raw::c_uint = 45;
pub const JVM_OPC_iaload: ::std::os::raw::c_uint = 46;
pub const JVM_OPC_laload: ::std::os::raw::c_uint = 47;
pub const JVM_OPC_faload: ::std::os::raw::c_uint = 48;
pub const JVM_OPC_daload: ::std::os::raw::c_uint = 49;
pub const JVM_OPC_aaload: ::std::os::raw::c_uint = 50;
pub const JVM_OPC_baload: ::std::os::raw::c_uint = 51;
pub const JVM_OPC_caload: ::std::os::raw::c_uint = 52;
pub const JVM_OPC_saload: ::std::os::raw::c_uint = 53;
pub const JVM_OPC_istore: ::std::os::raw::c_uint = 54;
pub const JVM_OPC_lstore: ::std::os::raw::c_uint = 55;
pub const JVM_OPC_fstore: ::std::os::raw::c_uint = 56;
pub const JVM_OPC_dstore: ::std::os::raw::c_uint = 57;
pub const JVM_OPC_astore: ::std::os::raw::c_uint = 58;
pub const JVM_OPC_istore_0: ::std::os::raw::c_uint = 59;
pub const JVM_OPC_istore_1: ::std::os::raw::c_uint = 60;
pub const JVM_OPC_istore_2: ::std::os::raw::c_uint = 61;
pub const JVM_OPC_istore_3: ::std::os::raw::c_uint = 62;
pub const JVM_OPC_lstore_0: ::std::os::raw::c_uint = 63;
pub const JVM_OPC_lstore_1: ::std::os::raw::c_uint = 64;
pub const JVM_OPC_lstore_2: ::std::os::raw::c_uint = 65;
pub const JVM_OPC_lstore_3: ::std::os::raw::c_uint = 66;
pub const JVM_OPC_fstore_0: ::std::os::raw::c_uint = 67;
pub const JVM_OPC_fstore_1: ::std::os::raw::c_uint = 68;
pub const JVM_OPC_fstore_2: ::std::os::raw::c_uint = 69;
pub const JVM_OPC_fstore_3: ::std::os::raw::c_uint = 70;
pub const JVM_OPC_dstore_0: ::std::os::raw::c_uint = 71;
pub const JVM_OPC_dstore_1: ::std::os::raw::c_uint = 72;
pub const JVM_OPC_dstore_2: ::std::os::raw::c_uint = 73;
pub const JVM_OPC_dstore_3: ::std::os::raw::c_uint = 74;
pub const JVM_OPC_astore_0: ::std::os::raw::c_uint = 75;
pub const JVM_OPC_astore_1: ::std::os::raw::c_uint = 76;
pub const JVM_OPC_astore_2: ::std::os::raw::c_uint = 77;
pub const JVM_OPC_astore_3: ::std::os::raw::c_uint = 78;
pub const JVM_OPC_iastore: ::std::os::raw::c_uint = 79;
pub const JVM_OPC_lastore: ::std::os::raw::c_uint = 80;
pub const JVM_OPC_fastore: ::std::os::raw::c_uint = 81;
pub const JVM_OPC_dastore: ::std::os::raw::c_uint = 82;
pub const JVM_OPC_aastore: ::std::os::raw::c_uint = 83;
pub const JVM_OPC_bastore: ::std::os::raw::c_uint = 84;
pub const JVM_OPC_castore: ::std::os::raw::c_uint = 85;
pub const JVM_OPC_sastore: ::std::os::raw::c_uint = 86;
pub const JVM_OPC_pop: ::std::os::raw::c_uint = 87;
pub const JVM_OPC_pop2: ::std::os::raw::c_uint = 88;
pub const JVM_OPC_dup: ::std::os::raw::c_uint = 89;
pub const JVM_OPC_dup_x1: ::std::os::raw::c_uint = 90;
pub const JVM_OPC_dup_x2: ::std::os::raw::c_uint = 91;
pub const JVM_OPC_dup2: ::std::os::raw::c_uint = 92;
pub const JVM_OPC_dup2_x1: ::std::os::raw::c_uint = 93;
pub const JVM_OPC_dup2_x2: ::std::os::raw::c_uint = 94;
pub const JVM_OPC_swap: ::std::os::raw::c_uint = 95;
pub const JVM_OPC_iadd: ::std::os::raw::c_uint = 96;
pub const JVM_OPC_ladd: ::std::os::raw::c_uint = 97;
pub const JVM_OPC_fadd: ::std::os::raw::c_uint = 98;
pub const JVM_OPC_dadd: ::std::os::raw::c_uint = 99;
pub const JVM_OPC_isub: ::std::os::raw::c_uint = 100;
pub const JVM_OPC_lsub: ::std::os::raw::c_uint = 101;
pub const JVM_OPC_fsub: ::std::os::raw::c_uint = 102;
pub const JVM_OPC_dsub: ::std::os::raw::c_uint = 103;
pub const JVM_OPC_imul: ::std::os::raw::c_uint = 104;
pub const JVM_OPC_lmul: ::std::os::raw::c_uint = 105;
pub const JVM_OPC_fmul: ::std::os::raw::c_uint = 106;
pub const JVM_OPC_dmul: ::std::os::raw::c_uint = 107;
pub const JVM_OPC_idiv: ::std::os::raw::c_uint = 108;
pub const JVM_OPC_ldiv: ::std::os::raw::c_uint = 109;
pub const JVM_OPC_fdiv: ::std::os::raw::c_uint = 110;
pub const JVM_OPC_ddiv: ::std::os::raw::c_uint = 111;
pub const JVM_OPC_irem: ::std::os::raw::c_uint = 112;
pub const JVM_OPC_lrem: ::std::os::raw::c_uint = 113;
pub const JVM_OPC_frem: ::std::os::raw::c_uint = 114;
pub const JVM_OPC_drem: ::std::os::raw::c_uint = 115;
pub const JVM_OPC_ineg: ::std::os::raw::c_uint = 116;
pub const JVM_OPC_lneg: ::std::os::raw::c_uint = 117;
pub const JVM_OPC_fneg: ::std::os::raw::c_uint = 118;
pub const JVM_OPC_dneg: ::std::os::raw::c_uint = 119;
pub const JVM_OPC_ishl: ::std::os::raw::c_uint = 120;
pub const JVM_OPC_lshl: ::std::os::raw::c_uint = 121;
pub const JVM_OPC_ishr: ::std::os::raw::c_uint = 122;
pub const JVM_OPC_lshr: ::std::os::raw::c_uint = 123;
pub const JVM_OPC_iushr: ::std::os::raw::c_uint = 124;
pub const JVM_OPC_lushr: ::std::os::raw::c_uint = 125;
pub const JVM_OPC_iand: ::std::os::raw::c_uint = 126;
pub const JVM_OPC_land: ::std::os::raw::c_uint = 127;
pub const JVM_OPC_ior: ::std::os::raw::c_uint = 128;
pub const JVM_OPC_lor: ::std::os::raw::c_uint = 129;
pub const JVM_OPC_ixor: ::std::os::raw::c_uint = 130;
pub const JVM_OPC_lxor: ::std::os::raw::c_uint = 131;
pub const JVM_OPC_iinc: ::std::os::raw::c_uint = 132;
pub const JVM_OPC_i2l: ::std::os::raw::c_uint = 133;
pub const JVM_OPC_i2f: ::std::os::raw::c_uint = 134;
pub const JVM_OPC_i2d: ::std::os::raw::c_uint = 135;
pub const JVM_OPC_l2i: ::std::os::raw::c_uint = 136;
pub const JVM_OPC_l2f: ::std::os::raw::c_uint = 137;
pub const JVM_OPC_l2d: ::std::os::raw::c_uint = 138;
pub const JVM_OPC_f2i: ::std::os::raw::c_uint = 139;
pub const JVM_OPC_f2l: ::std::os::raw::c_uint = 140;
pub const JVM_OPC_f2d: ::std::os::raw::c_uint = 141;
pub const JVM_OPC_d2i: ::std::os::raw::c_uint = 142;
pub const JVM_OPC_d2l: ::std::os::raw::c_uint = 143;
pub const JVM_OPC_d2f: ::std::os::raw::c_uint = 144;
pub const JVM_OPC_i2b: ::std::os::raw::c_uint = 145;
pub const JVM_OPC_i2c: ::std::os::raw::c_uint = 146;
pub const JVM_OPC_i2s: ::std::os::raw::c_uint = 147;
pub const JVM_OPC_lcmp: ::std::os::raw::c_uint = 148;
pub const JVM_OPC_fcmpl: ::std::os::raw::c_uint = 149;
pub const JVM_OPC_fcmpg: ::std::os::raw::c_uint = 150;
pub const JVM_OPC_dcmpl: ::std::os::raw::c_uint = 151;
pub const JVM_OPC_dcmpg: ::std::os::raw::c_uint = 152;
pub const JVM_OPC_ifeq: ::std::os::raw::c_uint = 153;
pub const JVM_OPC_ifne: ::std::os::raw::c_uint = 154;
pub const JVM_OPC_iflt: ::std::os::raw::c_uint = 155;
pub const JVM_OPC_ifge: ::std::os::raw::c_uint = 156;
pub const JVM_OPC_ifgt: ::std::os::raw::c_uint = 157;
pub const JVM_OPC_ifle: ::std::os::raw::c_uint = 158;
pub const JVM_OPC_if_icmpeq: ::std::os::raw::c_uint = 159;
pub const JVM_OPC_if_icmpne: ::std::os::raw::c_uint = 160;
pub const JVM_OPC_if_icmplt: ::std::os::raw::c_uint = 161;
pub const JVM_OPC_if_icmpge: ::std::os::raw::c_uint = 162;
pub const JVM_OPC_if_icmpgt: ::std::os::raw::c_uint = 163;
pub const JVM_OPC_if_icmple: ::std::os::raw::c_uint = 164;
pub const JVM_OPC_if_acmpeq: ::std::os::raw::c_uint = 165;
pub const JVM_OPC_if_acmpne: ::std::os::raw::c_uint = 166;
pub const JVM_OPC_goto: ::std::os::raw::c_uint = 167;
pub const JVM_OPC_jsr: ::std::os::raw::c_uint = 168;
pub const JVM_OPC_ret: ::std::os::raw::c_uint = 169;
pub const JVM_OPC_tableswitch: ::std::os::raw::c_uint = 170;
pub const JVM_OPC_lookupswitch: ::std::os::raw::c_uint = 171;
pub const JVM_OPC_ireturn: ::std::os::raw::c_uint = 172;
pub const JVM_OPC_lreturn: ::std::os::raw::c_uint = 173;
pub const JVM_OPC_freturn: ::std::os::raw::c_uint = 174;
pub const JVM_OPC_dreturn: ::std::os::raw::c_uint = 175;
pub const JVM_OPC_areturn: ::std::os::raw::c_uint = 176;
pub const JVM_OPC_return: ::std::os::raw::c_uint = 177;
pub const JVM_OPC_getstatic: ::std::os::raw::c_uint = 178;
pub const JVM_OPC_putstatic: ::std::os::raw::c_uint = 179;
pub const JVM_OPC_getfield: ::std::os::raw::c_uint = 180;
pub const JVM_OPC_putfield: ::std::os::raw::c_uint = 181;
pub const JVM_OPC_invokevirtual: ::std::os::raw::c_uint = 182;
pub const JVM_OPC_invokespecial: ::std::os::raw::c_uint = 183;
pub const JVM_OPC_invokestatic: ::std::os::raw::c_uint = 184;
pub const JVM_OPC_invokeinterface: ::std::os::raw::c_uint = 185;
pub const JVM_OPC_invokedynamic: ::std::os::raw::c_uint = 186;
pub const JVM_OPC_new: ::std::os::raw::c_uint = 187;
pub const JVM_OPC_newarray: ::std::os::raw::c_uint = 188;
pub const JVM_OPC_anewarray: ::std::os::raw::c_uint = 189;
pub const JVM_OPC_arraylength: ::std::os::raw::c_uint = 190;
pub const JVM_OPC_athrow: ::std::os::raw::c_uint = 191;
pub const JVM_OPC_checkcast: ::std::os::raw::c_uint = 192;
pub const JVM_OPC_instanceof: ::std::os::raw::c_uint = 193;
pub const JVM_OPC_monitorenter: ::std::os::raw::c_uint = 194;
pub const JVM_OPC_monitorexit: ::std::os::raw::c_uint = 195;
pub const JVM_OPC_wide: ::std::os::raw::c_uint = 196;
pub const JVM_OPC_multianewarray: ::std::os::raw::c_uint = 197;
pub const JVM_OPC_ifnull: ::std::os::raw::c_uint = 198;
pub const JVM_OPC_ifnonnull: ::std::os::raw::c_uint = 199;
pub const JVM_OPC_goto_w: ::std::os::raw::c_uint = 200;
pub const JVM_OPC_jsr_w: ::std::os::raw::c_uint = 201;
pub const JVM_OPC_MAX: ::std::os::raw::c_uint = 201;

pub type verifier_fn_t = ::std::option::Option<
	unsafe extern "C" fn(
		env: *mut JNIEnv,
		cb: jclass,
		msg_buf: *mut ::std::os::raw::c_char,
		buf_len: jint,
	) -> jboolean,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct method_size_info {
	pub code: ::std::os::raw::c_ulong,
	pub excs: ::std::os::raw::c_ulong,
	pub etab: ::std::os::raw::c_ulong,
	pub lnum: ::std::os::raw::c_ulong,
	pub lvar: ::std::os::raw::c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class_size_info {
	pub constants: ::std::os::raw::c_uint,
	pub fields: ::std::os::raw::c_uint,
	pub methods: ::std::os::raw::c_uint,
	pub interfaces: ::std::os::raw::c_uint,
	pub fields2: ::std::os::raw::c_uint,
	pub innerclasses: ::std::os::raw::c_uint,
	pub clinit: method_size_info,
	pub main: method_size_info,
}

pub type to_java_string_fn_t = ::std::option::Option<
	unsafe extern "C" fn(env: *mut JNIEnv, str_: *mut ::std::os::raw::c_char) -> jstring,
>;
pub type to_c_string_fn_t = ::std::option::Option<
	unsafe extern "C" fn(
		env: *mut JNIEnv,
		s: jstring,
		b: *mut jboolean,
	) -> *mut ::std::os::raw::c_char,
>;
pub type check_format_fn_t = ::std::option::Option<
	unsafe extern "C" fn(
		class_name: *mut ::std::os::raw::c_char,
		data: *mut ::std::os::raw::c_uchar,
		data_size: ::std::os::raw::c_uint,
		class_size: *mut class_size_info,
		message_buffer: *mut ::std::os::raw::c_char,
		buffer_length: jint,
		measure_only: jboolean,
		check_relaxed: jboolean,
	) -> jint,
>;
pub type canonicalize_fn_t = ::std::option::Option<
	unsafe extern "C" fn(
		env: *mut JNIEnv,
		orig: *mut ::std::os::raw::c_char,
		out: *mut ::std::os::raw::c_char,
		len: ::std::os::raw::c_int,
	) -> ::std::os::raw::c_int,
>;

extern "C" {
	pub fn JVM_GetLastErrorString(
		buf: *mut ::std::os::raw::c_char,
		len: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_NativePath(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}

extern "C" {
	pub fn JVM_Open(fname: *const ::std::os::raw::c_char, flags: jint, mode: jint) -> jint;
}

extern "C" {
	pub fn JVM_Close(fd: jint) -> jint;
}

extern "C" {
	pub fn JVM_Read(fd: jint, buf: *mut ::std::os::raw::c_char, nbytes: jint) -> jint;
}

extern "C" {
	pub fn JVM_Write(fd: jint, buf: *mut ::std::os::raw::c_char, nbytes: jint) -> jint;
}

extern "C" {
	pub fn JVM_Available(fd: jint, pbytes: *mut jlong) -> jint;
}

extern "C" {
	pub fn JVM_Lseek(fd: jint, offset: jlong, whence: jint) -> jlong;
}

extern "C" {
	pub fn JVM_SetLength(fd: jint, length: jlong) -> jint;
}

extern "C" {
	pub fn JVM_Sync(fd: jint) -> jint;
}

extern "C" {
	pub fn JVM_InitializeSocketLibrary() -> jint;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
	_unused: [u8; 0],
}

extern "C" {
	pub fn JVM_Socket(domain: jint, type_: jint, protocol: jint) -> jint;
}

extern "C" {
	pub fn JVM_SocketClose(fd: jint) -> jint;
}

extern "C" {
	pub fn JVM_SocketShutdown(fd: jint, howto: jint) -> jint;
}

extern "C" {
	pub fn JVM_Recv(fd: jint, buf: *mut ::std::os::raw::c_char, nBytes: jint, flags: jint) -> jint;
}

extern "C" {
	pub fn JVM_Send(fd: jint, buf: *mut ::std::os::raw::c_char, nBytes: jint, flags: jint) -> jint;
}

extern "C" {
	pub fn JVM_Timeout(fd: ::std::os::raw::c_int, timeout: ::std::os::raw::c_long) -> jint;
}

extern "C" {
	pub fn JVM_Listen(fd: jint, count: jint) -> jint;
}

extern "C" {
	pub fn JVM_Connect(fd: jint, him: *mut sockaddr, len: jint) -> jint;
}

extern "C" {
	pub fn JVM_Bind(fd: jint, him: *mut sockaddr, len: jint) -> jint;
}

extern "C" {
	pub fn JVM_Accept(fd: jint, him: *mut sockaddr, len: *mut jint) -> jint;
}

extern "C" {
	pub fn JVM_RecvFrom(
		fd: jint,
		buf: *mut ::std::os::raw::c_char,
		nBytes: ::std::os::raw::c_int,
		flags: ::std::os::raw::c_int,
		from: *mut sockaddr,
		fromlen: *mut ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_SendTo(
		fd: jint,
		buf: *mut ::std::os::raw::c_char,
		len: ::std::os::raw::c_int,
		flags: ::std::os::raw::c_int,
		to: *mut sockaddr,
		tolen: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_SocketAvailable(fd: jint, result: *mut jint) -> jint;
}

extern "C" {
	pub fn JVM_GetSockName(fd: jint, him: *mut sockaddr, len: *mut ::std::os::raw::c_int) -> jint;
}

extern "C" {
	pub fn JVM_GetSockOpt(
		fd: jint,
		level: ::std::os::raw::c_int,
		optname: ::std::os::raw::c_int,
		optval: *mut ::std::os::raw::c_char,
		optlen: *mut ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_SetSockOpt(
		fd: jint,
		level: ::std::os::raw::c_int,
		optname: ::std::os::raw::c_int,
		optval: *const ::std::os::raw::c_char,
		optlen: ::std::os::raw::c_int,
	) -> jint;
}

extern "C" {
	pub fn JVM_GetHostName(
		name: *mut ::std::os::raw::c_char,
		namelen: ::std::os::raw::c_int,
	) -> ::std::os::raw::c_int;
}

extern "C" {
	pub fn JVM_RawMonitorCreate() -> *mut ::std::os::raw::c_void;
}

extern "C" {
	pub fn JVM_RawMonitorDestroy(mon: *mut ::std::os::raw::c_void);
}

extern "C" {
	pub fn JVM_RawMonitorEnter(mon: *mut ::std::os::raw::c_void) -> jint;
}

extern "C" {
	pub fn JVM_RawMonitorExit(mon: *mut ::std::os::raw::c_void);
}

extern "C" {
	pub fn JVM_GetManagement(version: jint) -> *mut ::std::os::raw::c_void;
}

extern "C" {
	pub fn JVM_InitAgentProperties(env: *mut JNIEnv, agent_props: jobject) -> jobject;
}

extern "C" {
	pub fn JVM_GetTemporaryDirectory(env: *mut JNIEnv) -> jstring;
}

extern "C" {
	pub fn JVM_GetEnclosingMethodInfo(env: *mut JNIEnv, ofClass: jclass) -> jobjectArray;
}

pub const JAVA_THREAD_STATE_NEW: ::std::os::raw::c_uint = 0;
pub const JAVA_THREAD_STATE_RUNNABLE: ::std::os::raw::c_uint = 1;
pub const JAVA_THREAD_STATE_BLOCKED: ::std::os::raw::c_uint = 2;
pub const JAVA_THREAD_STATE_WAITING: ::std::os::raw::c_uint = 3;
pub const JAVA_THREAD_STATE_TIMED_WAITING: ::std::os::raw::c_uint = 4;
pub const JAVA_THREAD_STATE_TERMINATED: ::std::os::raw::c_uint = 5;
pub const JAVA_THREAD_STATE_COUNT: ::std::os::raw::c_uint = 6;

extern "C" {
	pub fn JVM_GetThreadStateValues(env: *mut JNIEnv, javaThreadState: jint) -> jintArray;
}

extern "C" {
	pub fn JVM_GetThreadStateNames(
		env: *mut JNIEnv,
		javaThreadState: jint,
		values: jintArray,
	) -> jobjectArray;
}

extern "C" {
	pub fn JVM_KnownToNotExist(
		env: *mut JNIEnv,
		loader: jobject,
		classname: *const ::std::os::raw::c_char,
	) -> jboolean;
}

extern "C" {
	pub fn JVM_GetResourceLookupCacheURLs(env: *mut JNIEnv, loader: jobject) -> jobjectArray;
}

extern "C" {
	pub fn JVM_GetResourceLookupCache(
		env: *mut JNIEnv,
		loader: jobject,
		resource_name: *const ::std::os::raw::c_char,
	) -> jintArray;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvm_version_info {
	pub jvm_version: ::std::os::raw::c_uint,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
	pub reserved2: ::std::os::raw::c_uint,
	pub _bitfield_2: __BindgenBitfieldUnit<[u8; 12usize], u8>,
}

impl jvm_version_info {
	#[inline]
	pub fn update_version(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
	}
	#[inline]
	pub fn set_update_version(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(0usize, 8u8, val as u64)
		}
	}
	#[inline]
	pub fn special_update_version(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
	}
	#[inline]
	pub fn set_special_update_version(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(8usize, 8u8, val as u64)
		}
	}
	#[inline]
	pub fn reserved1(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
	}
	#[inline]
	pub fn set_reserved1(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(16usize, 16u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_1(
		update_version: ::std::os::raw::c_uint,
		special_update_version: ::std::os::raw::c_uint,
		reserved1: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 8u8, {
			let update_version: u32 = unsafe { ::std::mem::transmute(update_version) };
			update_version as u64
		});
		__bindgen_bitfield_unit.set(8usize, 8u8, {
			let special_update_version: u32 =
				unsafe { ::std::mem::transmute(special_update_version) };
			special_update_version as u64
		});
		__bindgen_bitfield_unit.set(16usize, 16u8, {
			let reserved1: u32 = unsafe { ::std::mem::transmute(reserved1) };
			reserved1 as u64
		});
		__bindgen_bitfield_unit
	}
	#[inline]
	pub fn is_attach_supported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_is_attach_supported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_2.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_2(
		is_attach_supported: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 12usize], u8> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 12usize], u8> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let is_attach_supported: u32 = unsafe { ::std::mem::transmute(is_attach_supported) };
			is_attach_supported as u64
		});
		__bindgen_bitfield_unit
	}
}

extern "C" {
	pub fn JVM_GetVersionInfo(env: *mut JNIEnv, info: *mut jvm_version_info, info_size: size_t);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdk_version_info {
	pub jdk_version: ::std::os::raw::c_uint,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
	pub reserved2: ::std::os::raw::c_uint,
	pub _bitfield_2: __BindgenBitfieldUnit<[u8; 12usize], u8>,
}

impl jdk_version_info {
	#[inline]
	pub fn update_version(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
	}
	#[inline]
	pub fn set_update_version(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(0usize, 8u8, val as u64)
		}
	}
	#[inline]
	pub fn special_update_version(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
	}
	#[inline]
	pub fn set_special_update_version(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(8usize, 8u8, val as u64)
		}
	}
	#[inline]
	pub fn reserved1(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
	}
	#[inline]
	pub fn set_reserved1(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(16usize, 16u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_1(
		update_version: ::std::os::raw::c_uint,
		special_update_version: ::std::os::raw::c_uint,
		reserved1: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 8u8, {
			let update_version: u32 = unsafe { ::std::mem::transmute(update_version) };
			update_version as u64
		});
		__bindgen_bitfield_unit.set(8usize, 8u8, {
			let special_update_version: u32 =
				unsafe { ::std::mem::transmute(special_update_version) };
			special_update_version as u64
		});
		__bindgen_bitfield_unit.set(16usize, 16u8, {
			let reserved1: u32 = unsafe { ::std::mem::transmute(reserved1) };
			reserved1 as u64
		});
		__bindgen_bitfield_unit
	}
	#[inline]
	pub fn thread_park_blocker(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_thread_park_blocker(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_2.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn post_vm_init_hook_enabled(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_2.get(1usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_post_vm_init_hook_enabled(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_2.set(1usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn pending_list_uses_discovered_field(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_2.get(2usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_pending_list_uses_discovered_field(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_2.set(2usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_2(
		thread_park_blocker: ::std::os::raw::c_uint,
		post_vm_init_hook_enabled: ::std::os::raw::c_uint,
		pending_list_uses_discovered_field: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 12usize], u8> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 12usize], u8> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let thread_park_blocker: u32 = unsafe { ::std::mem::transmute(thread_park_blocker) };
			thread_park_blocker as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let post_vm_init_hook_enabled: u32 =
				unsafe { ::std::mem::transmute(post_vm_init_hook_enabled) };
			post_vm_init_hook_enabled as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let pending_list_uses_discovered_field: u32 =
				unsafe { ::std::mem::transmute(pending_list_uses_discovered_field) };
			pending_list_uses_discovered_field as u64
		});
		__bindgen_bitfield_unit
	}
}

pub type jdk_version_info_fn_t =
::std::option::Option<unsafe extern "C" fn(info: *mut jdk_version_info, info_size: size_t)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JDK1_1InitArgs {
	pub version: jint,
	pub properties: *mut *mut ::std::os::raw::c_char,
	pub checkSource: jint,
	pub nativeStackSize: jint,
	pub javaStackSize: jint,
	pub minHeapSize: jint,
	pub maxHeapSize: jint,
	pub verifyMode: jint,
	pub classpath: *mut ::std::os::raw::c_char,
	pub vfprintf: ::std::option::Option<
		unsafe extern "C" fn(
			fp: *mut FILE,
			format: *const ::std::os::raw::c_char,
			args: *mut __va_list_tag,
		) -> jint,
	>,
	pub exit: ::std::option::Option<unsafe extern "C" fn(code: jint)>,
	pub abort: ::std::option::Option<unsafe extern "C" fn()>,
	pub enableClassGC: jint,
	pub enableVerboseGC: jint,
	pub disableAsyncGC: jint,
	pub verbose: jint,
	pub debugging: jboolean,
	pub debugPort: jint,
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
