package main

/*
#cgo LDFLAGS: ./main.o -ldl
#include "./host.h"
*/
import "C"

import (
	"fmt"
)

func main() {
	var str C.struct_RocStr
	C.roc__mainForHost_1_exposed_generic(&str)
	fmt.Print(C.GoString(str.bytes))
}
