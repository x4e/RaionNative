#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::bitfield::__BindgenBitfieldUnit;
use jni::sys::{jvalue, jstring, jlong, jobjectArray, jint, jboolean, JNIEnv, jobject, jlongArray};

pub const JMM_VERSION_1: ::std::os::raw::c_uint = 536936448;
pub const JMM_VERSION_1_0: ::std::os::raw::c_uint = 536936448;
pub const JMM_VERSION_1_1: ::std::os::raw::c_uint = 536936704;
pub const JMM_VERSION_1_2: ::std::os::raw::c_uint = 536936960;
pub const JMM_VERSION_1_2_1: ::std::os::raw::c_uint = 536936961;
pub const JMM_VERSION_1_2_2: ::std::os::raw::c_uint = 536936962;
pub const JMM_VERSION: ::std::os::raw::c_uint = 536936963;

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct jmmOptionalSupport {
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u8>,
}

impl jmmOptionalSupport {
	#[inline]
	pub fn isLowMemoryDetectionSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isLowMemoryDetectionSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isCompilationTimeMonitoringSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isCompilationTimeMonitoringSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isThreadContentionMonitoringSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isThreadContentionMonitoringSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isCurrentThreadCpuTimeSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isCurrentThreadCpuTimeSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(3usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isOtherThreadCpuTimeSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isOtherThreadCpuTimeSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(4usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isBootClassPathSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isBootClassPathSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(5usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isObjectMonitorUsageSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isObjectMonitorUsageSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(6usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isSynchronizerUsageSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isSynchronizerUsageSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(7usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isThreadAllocatedMemorySupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isThreadAllocatedMemorySupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(8usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn isRemoteDiagnosticCommandsSupported(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_isRemoteDiagnosticCommandsSupported(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(9usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_1(
		isLowMemoryDetectionSupported: ::std::os::raw::c_uint,
		isCompilationTimeMonitoringSupported: ::std::os::raw::c_uint,
		isThreadContentionMonitoringSupported: ::std::os::raw::c_uint,
		isCurrentThreadCpuTimeSupported: ::std::os::raw::c_uint,
		isOtherThreadCpuTimeSupported: ::std::os::raw::c_uint,
		isBootClassPathSupported: ::std::os::raw::c_uint,
		isObjectMonitorUsageSupported: ::std::os::raw::c_uint,
		isSynchronizerUsageSupported: ::std::os::raw::c_uint,
		isThreadAllocatedMemorySupported: ::std::os::raw::c_uint,
		isRemoteDiagnosticCommandsSupported: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 4usize], u8> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u8> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let isLowMemoryDetectionSupported: u32 =
				unsafe { ::std::mem::transmute(isLowMemoryDetectionSupported) };
			isLowMemoryDetectionSupported as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let isCompilationTimeMonitoringSupported: u32 =
				unsafe { ::std::mem::transmute(isCompilationTimeMonitoringSupported) };
			isCompilationTimeMonitoringSupported as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let isThreadContentionMonitoringSupported: u32 =
				unsafe { ::std::mem::transmute(isThreadContentionMonitoringSupported) };
			isThreadContentionMonitoringSupported as u64
		});
		__bindgen_bitfield_unit.set(3usize, 1u8, {
			let isCurrentThreadCpuTimeSupported: u32 =
				unsafe { ::std::mem::transmute(isCurrentThreadCpuTimeSupported) };
			isCurrentThreadCpuTimeSupported as u64
		});
		__bindgen_bitfield_unit.set(4usize, 1u8, {
			let isOtherThreadCpuTimeSupported: u32 =
				unsafe { ::std::mem::transmute(isOtherThreadCpuTimeSupported) };
			isOtherThreadCpuTimeSupported as u64
		});
		__bindgen_bitfield_unit.set(5usize, 1u8, {
			let isBootClassPathSupported: u32 =
				unsafe { ::std::mem::transmute(isBootClassPathSupported) };
			isBootClassPathSupported as u64
		});
		__bindgen_bitfield_unit.set(6usize, 1u8, {
			let isObjectMonitorUsageSupported: u32 =
				unsafe { ::std::mem::transmute(isObjectMonitorUsageSupported) };
			isObjectMonitorUsageSupported as u64
		});
		__bindgen_bitfield_unit.set(7usize, 1u8, {
			let isSynchronizerUsageSupported: u32 =
				unsafe { ::std::mem::transmute(isSynchronizerUsageSupported) };
			isSynchronizerUsageSupported as u64
		});
		__bindgen_bitfield_unit.set(8usize, 1u8, {
			let isThreadAllocatedMemorySupported: u32 =
				unsafe { ::std::mem::transmute(isThreadAllocatedMemorySupported) };
			isThreadAllocatedMemorySupported as u64
		});
		__bindgen_bitfield_unit.set(9usize, 1u8, {
			let isRemoteDiagnosticCommandsSupported: u32 =
				unsafe { ::std::mem::transmute(isRemoteDiagnosticCommandsSupported) };
			isRemoteDiagnosticCommandsSupported as u64
		});
		__bindgen_bitfield_unit
	}
}

pub const jmmLongAttribute_JMM_CLASS_LOADED_COUNT: jmmLongAttribute = 1;
pub const jmmLongAttribute_JMM_CLASS_UNLOADED_COUNT: jmmLongAttribute = 2;
pub const jmmLongAttribute_JMM_THREAD_TOTAL_COUNT: jmmLongAttribute = 3;
pub const jmmLongAttribute_JMM_THREAD_LIVE_COUNT: jmmLongAttribute = 4;
pub const jmmLongAttribute_JMM_THREAD_PEAK_COUNT: jmmLongAttribute = 5;
pub const jmmLongAttribute_JMM_THREAD_DAEMON_COUNT: jmmLongAttribute = 6;
pub const jmmLongAttribute_JMM_JVM_INIT_DONE_TIME_MS: jmmLongAttribute = 7;
pub const jmmLongAttribute_JMM_COMPILE_TOTAL_TIME_MS: jmmLongAttribute = 8;
pub const jmmLongAttribute_JMM_GC_TIME_MS: jmmLongAttribute = 9;
pub const jmmLongAttribute_JMM_GC_COUNT: jmmLongAttribute = 10;
pub const jmmLongAttribute_JMM_JVM_UPTIME_MS: jmmLongAttribute = 11;
pub const jmmLongAttribute_JMM_INTERNAL_ATTRIBUTE_INDEX: jmmLongAttribute = 100;
pub const jmmLongAttribute_JMM_CLASS_LOADED_BYTES: jmmLongAttribute = 101;
pub const jmmLongAttribute_JMM_CLASS_UNLOADED_BYTES: jmmLongAttribute = 102;
pub const jmmLongAttribute_JMM_TOTAL_CLASSLOAD_TIME_MS: jmmLongAttribute = 103;
pub const jmmLongAttribute_JMM_VM_GLOBAL_COUNT: jmmLongAttribute = 104;
pub const jmmLongAttribute_JMM_SAFEPOINT_COUNT: jmmLongAttribute = 105;
pub const jmmLongAttribute_JMM_TOTAL_SAFEPOINTSYNC_TIME_MS: jmmLongAttribute = 106;
pub const jmmLongAttribute_JMM_TOTAL_STOPPED_TIME_MS: jmmLongAttribute = 107;
pub const jmmLongAttribute_JMM_TOTAL_APP_TIME_MS: jmmLongAttribute = 108;
pub const jmmLongAttribute_JMM_VM_THREAD_COUNT: jmmLongAttribute = 109;
pub const jmmLongAttribute_JMM_CLASS_INIT_TOTAL_COUNT: jmmLongAttribute = 110;
pub const jmmLongAttribute_JMM_CLASS_INIT_TOTAL_TIME_MS: jmmLongAttribute = 111;
pub const jmmLongAttribute_JMM_METHOD_DATA_SIZE_BYTES: jmmLongAttribute = 112;
pub const jmmLongAttribute_JMM_CLASS_VERIFY_TOTAL_TIME_MS: jmmLongAttribute = 113;
pub const jmmLongAttribute_JMM_SHARED_CLASS_LOADED_COUNT: jmmLongAttribute = 114;
pub const jmmLongAttribute_JMM_SHARED_CLASS_UNLOADED_COUNT: jmmLongAttribute = 115;
pub const jmmLongAttribute_JMM_SHARED_CLASS_LOADED_BYTES: jmmLongAttribute = 116;
pub const jmmLongAttribute_JMM_SHARED_CLASS_UNLOADED_BYTES: jmmLongAttribute = 117;
pub const jmmLongAttribute_JMM_OS_ATTRIBUTE_INDEX: jmmLongAttribute = 200;
pub const jmmLongAttribute_JMM_OS_PROCESS_ID: jmmLongAttribute = 201;
pub const jmmLongAttribute_JMM_OS_MEM_TOTAL_PHYSICAL_BYTES: jmmLongAttribute = 202;
pub const jmmLongAttribute_JMM_GC_EXT_ATTRIBUTE_INFO_SIZE: jmmLongAttribute = 401;

pub type jmmLongAttribute = ::std::os::raw::c_uint;

pub const jmmBoolAttribute_JMM_VERBOSE_GC: jmmBoolAttribute = 21;
pub const jmmBoolAttribute_JMM_VERBOSE_CLASS: jmmBoolAttribute = 22;
pub const jmmBoolAttribute_JMM_THREAD_CONTENTION_MONITORING: jmmBoolAttribute = 23;
pub const jmmBoolAttribute_JMM_THREAD_CPU_TIME: jmmBoolAttribute = 24;
pub const jmmBoolAttribute_JMM_THREAD_ALLOCATED_MEMORY: jmmBoolAttribute = 25;

pub type jmmBoolAttribute = ::std::os::raw::c_uint;

pub const JMM_THREAD_STATE_FLAG_SUSPENDED: ::std::os::raw::c_uint = 1048576;
pub const JMM_THREAD_STATE_FLAG_NATIVE: ::std::os::raw::c_uint = 4194304;
pub const jmmStatisticType_JMM_STAT_PEAK_THREAD_COUNT: jmmStatisticType = 801;
pub const jmmStatisticType_JMM_STAT_THREAD_CONTENTION_COUNT: jmmStatisticType = 802;
pub const jmmStatisticType_JMM_STAT_THREAD_CONTENTION_TIME: jmmStatisticType = 803;
pub const jmmStatisticType_JMM_STAT_THREAD_CONTENTION_STAT: jmmStatisticType = 804;
pub const jmmStatisticType_JMM_STAT_PEAK_POOL_USAGE: jmmStatisticType = 805;
pub const jmmStatisticType_JMM_STAT_GC_STAT: jmmStatisticType = 806;

pub type jmmStatisticType = ::std::os::raw::c_uint;

pub const jmmThresholdType_JMM_USAGE_THRESHOLD_HIGH: jmmThresholdType = 901;
pub const jmmThresholdType_JMM_USAGE_THRESHOLD_LOW: jmmThresholdType = 902;
pub const jmmThresholdType_JMM_COLLECTION_USAGE_THRESHOLD_HIGH: jmmThresholdType = 903;
pub const jmmThresholdType_JMM_COLLECTION_USAGE_THRESHOLD_LOW: jmmThresholdType = 904;

pub type jmmThresholdType = ::std::os::raw::c_uint;

pub const jmmVMGlobalType_JMM_VMGLOBAL_TYPE_UNKNOWN: jmmVMGlobalType = 0;
pub const jmmVMGlobalType_JMM_VMGLOBAL_TYPE_JBOOLEAN: jmmVMGlobalType = 1;
pub const jmmVMGlobalType_JMM_VMGLOBAL_TYPE_JSTRING: jmmVMGlobalType = 2;
pub const jmmVMGlobalType_JMM_VMGLOBAL_TYPE_JLONG: jmmVMGlobalType = 3;
pub const jmmVMGlobalType_JMM_VMGLOBAL_TYPE_JDOUBLE: jmmVMGlobalType = 4;

pub type jmmVMGlobalType = ::std::os::raw::c_uint;

pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_DEFAULT: jmmVMGlobalOrigin = 1;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_COMMAND_LINE: jmmVMGlobalOrigin = 2;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_MANAGEMENT: jmmVMGlobalOrigin = 3;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_ENVIRON_VAR: jmmVMGlobalOrigin = 4;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_CONFIG_FILE: jmmVMGlobalOrigin = 5;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_ERGONOMIC: jmmVMGlobalOrigin = 6;
pub const jmmVMGlobalOrigin_JMM_VMGLOBAL_ORIGIN_OTHER: jmmVMGlobalOrigin = 99;

pub type jmmVMGlobalOrigin = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jmmVMGlobal {
	pub name: jstring,
	pub value: jvalue,
	pub type_: jmmVMGlobalType,
	pub origin: jmmVMGlobalOrigin,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
	pub reserved1: *mut ::std::os::raw::c_void,
	pub reserved2: *mut ::std::os::raw::c_void,
}

impl jmmVMGlobal {
	#[inline]
	pub fn writeable(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_writeable(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn external(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_external(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub fn reserved(&self) -> ::std::os::raw::c_uint {
		unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 30u8) as u32) }
	}
	#[inline]
	pub fn set_reserved(&mut self, val: ::std::os::raw::c_uint) {
		unsafe {
			let val: u32 = ::std::mem::transmute(val);
			self._bitfield_1.set(2usize, 30u8, val as u64)
		}
	}
	#[inline]
	pub fn new_bitfield_1(
		writeable: ::std::os::raw::c_uint,
		external: ::std::os::raw::c_uint,
		reserved: ::std::os::raw::c_uint,
	) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let writeable: u32 = unsafe { ::std::mem::transmute(writeable) };
			writeable as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let external: u32 = unsafe { ::std::mem::transmute(external) };
			external as u64
		});
		__bindgen_bitfield_unit.set(2usize, 30u8, {
			let reserved: u32 = unsafe { ::std::mem::transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jmmExtAttributeInfo {
	pub name: *const ::std::os::raw::c_char,
	pub type_: ::std::os::raw::c_char,
	pub description: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jmmGCStat {
	pub gc_index: jlong,
	pub start_time: jlong,
	pub end_time: jlong,
	pub usage_before_gc: jobjectArray,
	pub usage_after_gc: jobjectArray,
	pub gc_ext_attribute_values_size: jint,
	pub gc_ext_attribute_values: *mut jvalue,
	pub num_gc_ext_attributes: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dcmdInfo {
	pub name: *const ::std::os::raw::c_char,
	pub description: *const ::std::os::raw::c_char,
	pub impact: *const ::std::os::raw::c_char,
	pub permission_class: *const ::std::os::raw::c_char,
	pub permission_name: *const ::std::os::raw::c_char,
	pub permission_action: *const ::std::os::raw::c_char,
	pub num_arguments: ::std::os::raw::c_int,
	pub enabled: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dcmdArgInfo {
	pub name: *const ::std::os::raw::c_char,
	pub description: *const ::std::os::raw::c_char,
	pub type_: *const ::std::os::raw::c_char,
	pub default_string: *const ::std::os::raw::c_char,
	pub mandatory: jboolean,
	pub option: jboolean,
	pub multiple: jboolean,
	pub position: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jmmInterface_1_ {
	pub reserved1: *mut ::std::os::raw::c_void,
	pub reserved2: *mut ::std::os::raw::c_void,
	pub GetVersion: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jint>,
	pub GetOptionalSupport: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, support_ptr: *mut jmmOptionalSupport) -> jint,
	>,
	pub GetInputArguments: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jobject>,
	pub GetThreadInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			ids: jlongArray,
			maxDepth: jint,
			infoArray: jobjectArray,
		) -> jint,
	>,
	pub GetInputArgumentArray:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jobjectArray>,
	pub GetMemoryPools:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, mgr: jobject) -> jobjectArray>,
	pub GetMemoryManagers: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, pool: jobject) -> jobjectArray,
	>,
	pub GetMemoryPoolUsage:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, pool: jobject) -> jobject>,
	pub GetPeakMemoryPoolUsage:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, pool: jobject) -> jobject>,
	pub GetThreadAllocatedMemory: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, ids: jlongArray, sizeArray: jlongArray),
	>,
	pub GetMemoryUsage:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, heap: jboolean) -> jobject>,
	pub GetLongAttribute: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, att: jmmLongAttribute) -> jlong,
	>,
	pub GetBoolAttribute: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, att: jmmBoolAttribute) -> jboolean,
	>,
	pub SetBoolAttribute: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, att: jmmBoolAttribute, flag: jboolean) -> jboolean,
	>,
	pub GetLongAttributes: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			obj: jobject,
			atts: *mut jmmLongAttribute,
			count: jint,
			result: *mut jlong,
		) -> jint,
	>,
	pub FindCircularBlockedThreads:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jobjectArray>,
	pub GetThreadCpuTime:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, thread_id: jlong) -> jlong>,
	pub GetVMGlobalNames:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jobjectArray>,
	pub GetVMGlobals: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			names: jobjectArray,
			globals: *mut jmmVMGlobal,
			count: jint,
		) -> jint,
	>,
	pub GetInternalThreadTimes: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, names: jobjectArray, times: jlongArray) -> jint,
	>,
	pub ResetStatistic: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, obj: jvalue, type_: jmmStatisticType) -> jboolean,
	>,
	pub SetPoolSensor: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			pool: jobject,
			type_: jmmThresholdType,
			sensor: jobject,
		),
	>,
	pub SetPoolThreshold: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			pool: jobject,
			type_: jmmThresholdType,
			threshold: jlong,
		) -> jlong,
	>,
	pub GetPoolCollectionUsage:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, pool: jobject) -> jobject>,
	pub GetGCExtAttributeInfo: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			mgr: jobject,
			ext_info: *mut jmmExtAttributeInfo,
			count: jint,
		) -> jint,
	>,
	pub GetLastGCStat: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, mgr: jobject, gc_stat: *mut jmmGCStat),
	>,
	pub GetThreadCpuTimeWithKind: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			thread_id: jlong,
			user_sys_cpu_time: jboolean,
		) -> jlong,
	>,
	pub GetThreadCpuTimesWithKind: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			ids: jlongArray,
			timeArray: jlongArray,
			user_sys_cpu_time: jboolean,
		),
	>,
	pub DumpHeap0: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, outputfile: jstring, live: jboolean) -> jint,
	>,
	pub FindDeadlocks: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, object_monitors_only: jboolean) -> jobjectArray,
	>,
	pub SetVMGlobal: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, flag_name: jstring, new_value: jvalue),
	>,
	pub reserved6: *mut ::std::os::raw::c_void,
	pub DumpThreads: ::std::option::Option<
		unsafe extern "C" fn(
			env: *mut JNIEnv,
			ids: jlongArray,
			lockedMonitors: jboolean,
			lockedSynchronizers: jboolean,
		) -> jobjectArray,
	>,
	pub SetGCNotificationEnabled: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, mgr: jobject, enabled: jboolean),
	>,
	pub GetDiagnosticCommands:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jobjectArray>,
	pub GetDiagnosticCommandInfo: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, cmds: jobjectArray, infoArray: *mut dcmdInfo),
	>,
	pub GetDiagnosticCommandArgumentsInfo: ::std::option::Option<
		unsafe extern "C" fn(env: *mut JNIEnv, commandName: jstring, infoArray: *mut dcmdArgInfo),
	>,
	pub ExecuteDiagnosticCommand:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, command: jstring) -> jstring>,
	pub SetDiagnosticFrameworkNotificationEnabled:
	::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, enabled: jboolean)>,
}

pub type JmmInterface = jmmInterface_1_;
pub type __builtin_va_list = [__va_list_tag; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
	pub gp_offset: ::std::os::raw::c_uint,
	pub fp_offset: ::std::os::raw::c_uint,
	pub overflow_arg_area: *mut ::std::os::raw::c_void,
	pub reg_save_area: *mut ::std::os::raw::c_void,
}
