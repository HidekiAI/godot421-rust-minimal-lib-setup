#!/bin/bash
# Assumes dir matches lib-name!
# arg1: platform
# arg2...: libs
_ARG1_PLATFORM="windows"
_ARG2_LIBS="lib1 lib2"
_GODOT_PROJECT="../godot/godot-rust-hello_world/"
_GODOT_VERSION="4.2"
if [ x"$1" != x"" ]; then
  _ARG1_PLATFORM=$1
  shift

  if [ x"$1" != x"" ]; then
    _ARG2_LIBS=$@
  fi
fi


# Apparently, for 4.2.1, the format has changed to "${_PLATFORM}.template_${_TARGET}.${_ARCH}.${_PLATFORM_EXT}"
# ${_PLATFORM}.${_TARGET}.${_ARCH} = "res://../../rust/${_LIB_NAME}/target/${_TARGET}/lib${_LIB_NAME}.${_PLATFORM}.template_${_TARGET}.${_ARCH}.${_PLATFORM_EXT}"
# ${_PLATFORM}.${_ARCH}            = "res://../../rust/${_LIB_NAME}/target/${_TARGET}/lib${_LIB_NAME}.${_PLATFORM}.template_${_TARGET}.${_ARCH}.${_PLATFORM_EXT}"
#
# Aarg1: libname (i.e. "lib_rust_1")
# Arg2: (Optional) Platform; if not passed, assumes Linux
# Arg3: (Optional) target; if not passed, assumes Debug
# Arg4: (Optional) Arch; if not passed, assumes x86_64
function make_libname() {
    if [ x"$1" == x"" ]; then
        echo "#ERROR: Must pass libname as arg1"
        exit -666
    fi
    _ARG1_LIBNAME=$1
    shift
    _LIBNAME="lib${_ARG1_LIBNAME}"

    # defaults:
    _PLATFORM="linux"
    _ARCH="x86_64"
    _PLATFORM_EXT="so"
    _TARGET="debug"

    if [ x"$1" != x"" ]; then
        _PLATFORM=$1
        shift
        if [ x"$1" != x"" ]; then
            _TARGET=$1
            shift
        fi
    fi

    if [ "${_PLATFORM}" == "windows" ]; then
        _LIBNAME="${_ARG1_LIBNAME}"
        _PLATFORM_EXT="dll"
    elif [ "${_PLATFORM}" == "macos" ]; then
        echo "# NOTE/WARNING: Don't really care to test macos x86_64 and arm...  so fix this yourself :P"
        _PLATFORM_EXT="dylib"
    fi

    echo "target/${_TARGET}/${_LIB_NAME}.${_PLATFORM}.template_${_TARGET}.${_ARCH}.${_PLATFORM_EXT}"
}


# Arg1: libname
# Arg2: DEBUG target
# Arg3: Release target
# Arg4: Platform
function make_gdext() {
    if [ x"$1" == x"" ]; then
        echo "#ERROR: Must pass libname as arg1"
        exit -666
    fi

    echo ""
    echo "[configuration]"
    echo "entry_symbol = \"gdext_rust_init\""
    echo "compatibility_minimum = ${_GODOT_VERSION}"
    echo ""
    echo "[libraries]"

    if [ "$4" == "linux" ]; then
        echo "linux.debug.x86_64 =     \"res://../../rust/$1/$2\""
        echo "linux.release.x86_64 =   \"res://../../rust/$1/$3\""
        echo "linux.x86_64 =           \"res://../../rust/$1/$2\""
    elif [ "$4" == "windows" ]; then
        echo "windows.debug.x86_64 =   \"res://../../rust/$1/$2\""
        echo "windows.release.x86_64 = \"res://../../rust/$1/$3\""
        echo "windows.x86_64 =         \"res://../../rust/$1/$2\""
    elif [ "$4" == "macos" ]; then
        echo "macos.debug =            \"res://../../rust/$1/$2\""
        echo "macos.release =          \"res://../../rust/$1/$3\""
        echo "macos =                  \"res://../../rust/$1/$2\""
    elif [ "$4" == "macos.arm64" ]; then
        echo "macos.debug.arm64 =      \"res://../../rust/$1/$2\""
        echo "macos.release.arm64 =    \"res://../../rust/$1/$3\""
        echo "macos.arm64 =            \"res://../../rust/$1/$2\""
    fi
}

for _LIB_NAME in ${_ARG2_LIBS}; do
  echo -e "\nBuilding ${_LIB_NAME}"
  pushd .

  cd ${_LIB_NAME}
  cargo build
  _SRC="target/debug/lib${_LIB_NAME}.so"
  _T=$(make_libname ${_LIB_NAME})
  if [ ${_ARG1_PLATFORM} = "windows" ]; then
    _SRC="target/debug/${_LIB_NAME}.dll"
      _T=$(make_libname ${_LIB_NAME} "windows" "debug")
  fi
  echo -e "\n ################# GDExtension: ${_SRC}"
  cp -v ${_SRC} ${_T}
  ls -lAh ${_T}
  _TDEBUG=${_T}

  cargo build --release
  _SRC="target/release/lib${_LIB_NAME}.so"
  _T=$(make_libname ${_LIB_NAME} "linux" "release") 
  _SRC="target/release/lib${_LIB_NAME}"
  if [ ${_ARG1_PLATFORM} = "windows" ]; then
    _SRC="target/release/${_LIB_NAME}.dll"
    _T=$(make_libname ${_LIB_NAME} "windows" "release") 
  fi
  echo "################# GDExtension: ${_SRC}"
  cp -v ${_SRC} ${_T}
  ls -lAh ${_T}
  _TRELEASE=${_T}

  popd

  make_gdext ${_LIB_NAME} ${_TDEBUG} ${_TRELEASE} ${_ARG1_PLATFORM} > "${_GODOT_PROJECT}/${_LIB_NAME}.gdextension"
  cat "${_GODOT_PROJECT}/${_LIB_NAME}.gdextension"
done
