package main

/*
#cgo LDFLAGS: ./main.o -ldl
#include "./host.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	var str C.struct_RocStr
	C.roc__mainForHost_1_exposed_generic(&str)
	fmt.Println(readRocStr(str))
}

const is64Bit = uint64(^uintptr(0)) == ^uint64(0)

func readRocStr(str C.struct_RocStr) string {
	if int(str.data[2]) < 0 {
		ptr := (*byte)(unsafe.Pointer(unsafe.SliceData(str.data[:])))

		byteLen := 12
		if is64Bit {
			byteLen = 24
		}

		got := unsafe.String(ptr, byteLen)
		len := got[byteLen-1] ^ 128
		return got[:len]
	}
	ptr := (*byte)(unsafe.Pointer(uintptr(str.data[0])))
	return unsafe.String(ptr, str.data[1])
}

//export roc_alloc
func roc_alloc(size C.ulong, alignment int) unsafe.Pointer {
	return C.malloc(size)
}

//export roc_realloc
func roc_realloc(ptr unsafe.Pointer, newSize, _ C.ulong, alignment int) unsafe.Pointer {
	return C.realloc(ptr, newSize)
}

//export roc_dealloc
func roc_dealloc(ptr unsafe.Pointer, alignment int) {
	C.free(ptr)
}
