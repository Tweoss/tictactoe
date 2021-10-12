"use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.tictactoe = factory();
    }
}( this, function() {
    return (function( module_factory ) {
        var instance = module_factory();

        if( typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string" ) {
            var fs = require( "fs" );
            var path = require( "path" );
            var wasm_path = path.join( __dirname, "tictactoe.wasm" );
            var buffer = fs.readFileSync( wasm_path );
            var mod = new WebAssembly.Module( buffer );
            var wasm_instance = new WebAssembly.Instance( mod, instance.imports );
            return instance.initialize( wasm_instance );
        } else {
            var file = fetch( "tictactoe.wasm", {credentials: "same-origin"} );

            var wasm_instance = ( typeof WebAssembly.instantiateStreaming === "function"
                ? WebAssembly.instantiateStreaming( file, instance.imports )
                    .then( function( result ) { return result.instance; } )

                : file
                    .then( function( response ) { return response.arrayBuffer(); } )
                    .then( function( bytes ) { return WebAssembly.compile( bytes ); } )
                    .then( function( mod ) { return WebAssembly.instantiate( mod, instance.imports ) } ) );

            return wasm_instance
                .then( function( wasm_instance ) {
                    var exports = instance.initialize( wasm_instance );
                    console.log( "Finished loading Rust wasm module 'tictactoe'" );
                    return exports;
                })
                .catch( function( error ) {
                    console.log( "Error loading Rust wasm module 'tictactoe':", error );
                    throw error;
                });
        }
    }( function() {
    var Module = {};

    Module.STDWEB_PRIVATE = {};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.to_utf8 = function to_utf8( str, addr ) {
    var HEAPU8 = Module.HEAPU8;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            HEAPU8[ addr++ ] = u;
        } else if( u <= 0x7FF ) {
            HEAPU8[ addr++ ] = 0xC0 | (u >> 6);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0xFFFF ) {
            HEAPU8[ addr++ ] = 0xE0 | (u >> 12);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x1FFFFF ) {
            HEAPU8[ addr++ ] = 0xF0 | (u >> 18);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x3FFFFFF ) {
            HEAPU8[ addr++ ] = 0xF8 | (u >> 24);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else {
            HEAPU8[ addr++ ] = 0xFC | (u >> 30);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        }
    }
};

Module.STDWEB_PRIVATE.noop = function() {};
Module.STDWEB_PRIVATE.to_js = function to_js( address ) {
    var kind = Module.HEAPU8[ address + 12 ];
    if( kind === 0 ) {
        return undefined;
    } else if( kind === 1 ) {
        return null;
    } else if( kind === 2 ) {
        return Module.HEAP32[ address / 4 ];
    } else if( kind === 3 ) {
        return Module.HEAPF64[ address / 8 ];
    } else if( kind === 4 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        return Module.STDWEB_PRIVATE.to_js_string( pointer, length );
    } else if( kind === 5 ) {
        return false;
    } else if( kind === 6 ) {
        return true;
    } else if( kind === 7 ) {
        var pointer = Module.STDWEB_PRIVATE.arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var output = [];
        for( var i = 0; i < length; ++i ) {
            output.push( Module.STDWEB_PRIVATE.to_js( pointer + i * 16 ) );
        }
        return output;
    } else if( kind === 8 ) {
        var arena = Module.STDWEB_PRIVATE.arena;
        var value_array_pointer = arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var key_array_pointer = arena + Module.HEAPU32[ (address + 8) / 4 ];
        var output = {};
        for( var i = 0; i < length; ++i ) {
            var key_pointer = Module.HEAPU32[ (key_array_pointer + i * 8) / 4 ];
            var key_length = Module.HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];
            var key = Module.STDWEB_PRIVATE.to_js_string( key_pointer, key_length );
            var value = Module.STDWEB_PRIVATE.to_js( value_array_pointer + i * 16 );
            output[ key ] = value;
        }
        return output;
    } else if( kind === 9 ) {
        return Module.STDWEB_PRIVATE.acquire_js_reference( Module.HEAP32[ address / 4 ] );
    } else if( kind === 10 || kind === 12 || kind === 13 ) {
        var adapter_pointer = Module.HEAPU32[ address / 4 ];
        var pointer = Module.HEAPU32[ (address + 4) / 4 ];
        var deallocator_pointer = Module.HEAPU32[ (address + 8) / 4 ];
        var num_ongoing_calls = 0;
        var drop_queued = false;
        var output = function() {
            if( pointer === 0 || drop_queued === true ) {
                if (kind === 10) {
                    throw new ReferenceError( "Already dropped Rust function called!" );
                } else if (kind === 12) {
                    throw new ReferenceError( "Already dropped FnMut function called!" );
                } else {
                    throw new ReferenceError( "Already called or dropped FnOnce function called!" );
                }
            }

            var function_pointer = pointer;
            if (kind === 13) {
                output.drop = Module.STDWEB_PRIVATE.noop;
                pointer = 0;
            }

            if (num_ongoing_calls !== 0) {
                if (kind === 12 || kind === 13) {
                    throw new ReferenceError( "FnMut function called multiple times concurrently!" );
                }
            }

            var args = Module.STDWEB_PRIVATE.alloc( 16 );
            Module.STDWEB_PRIVATE.serialize_array( args, arguments );

            try {
                num_ongoing_calls += 1;
                Module.STDWEB_PRIVATE.dyncall( "vii", adapter_pointer, [function_pointer, args] );
                var result = Module.STDWEB_PRIVATE.tmp;
                Module.STDWEB_PRIVATE.tmp = null;
            } finally {
                num_ongoing_calls -= 1;
            }

            if( drop_queued === true && num_ongoing_calls === 0 ) {
                output.drop();
            }

            return result;
        };

        output.drop = function() {
            if (num_ongoing_calls !== 0) {
                drop_queued = true;
                return;
            }

            output.drop = Module.STDWEB_PRIVATE.noop;
            var function_pointer = pointer;
            pointer = 0;

            if (function_pointer != 0) {
                Module.STDWEB_PRIVATE.dyncall( "vi", deallocator_pointer, [function_pointer] );
            }
        };

        return output;
    } else if( kind === 14 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var array_kind = Module.HEAPU32[ (address + 8) / 4 ];
        var pointer_end = pointer + length;

        switch( array_kind ) {
            case 0:
                return Module.HEAPU8.subarray( pointer, pointer_end );
            case 1:
                return Module.HEAP8.subarray( pointer, pointer_end );
            case 2:
                return Module.HEAPU16.subarray( pointer, pointer_end );
            case 3:
                return Module.HEAP16.subarray( pointer, pointer_end );
            case 4:
                return Module.HEAPU32.subarray( pointer, pointer_end );
            case 5:
                return Module.HEAP32.subarray( pointer, pointer_end );
            case 6:
                return Module.HEAPF32.subarray( pointer, pointer_end );
            case 7:
                return Module.HEAPF64.subarray( pointer, pointer_end );
        }
    } else if( kind === 15 ) {
        return Module.STDWEB_PRIVATE.get_raw_value( Module.HEAPU32[ address / 4 ] );
    }
};

Module.STDWEB_PRIVATE.serialize_object = function serialize_object( address, value ) {
    var keys = Object.keys( value );
    var length = keys.length;
    var key_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 8 );
    var value_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 8;
    Module.HEAPU32[ address / 4 ] = value_array_pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    Module.HEAPU32[ (address + 8) / 4 ] = key_array_pointer;
    for( var i = 0; i < length; ++i ) {
        var key = keys[ i ];
        var key_address = key_array_pointer + i * 8;
        Module.STDWEB_PRIVATE.to_utf8_string( key_address, key );

        Module.STDWEB_PRIVATE.from_js( value_array_pointer + i * 16, value[ key ] );
    }
};

Module.STDWEB_PRIVATE.serialize_array = function serialize_array( address, value ) {
    var length = value.length;
    var pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 7;
    Module.HEAPU32[ address / 4 ] = pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    for( var i = 0; i < length; ++i ) {
        Module.STDWEB_PRIVATE.from_js( pointer + i * 16, value[ i ] );
    }
};

// New browsers and recent Node
var cachedEncoder = ( typeof TextEncoder === "function"
    ? new TextEncoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextEncoder === "function"
        ? new util.TextEncoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedEncoder != null ) {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var buffer = cachedEncoder.encode( value );
        var length = buffer.length;
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.HEAPU8.set( buffer, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };

} else {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var length = Module.STDWEB_PRIVATE.utf8_len( value );
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.STDWEB_PRIVATE.to_utf8( value, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };
}

Module.STDWEB_PRIVATE.from_js = function from_js( address, value ) {
    var kind = Object.prototype.toString.call( value );
    if( kind === "[object String]" ) {
        Module.HEAPU8[ address + 12 ] = 4;
        Module.STDWEB_PRIVATE.to_utf8_string( address, value );
    } else if( kind === "[object Number]" ) {
        if( value === (value|0) ) {
            Module.HEAPU8[ address + 12 ] = 2;
            Module.HEAP32[ address / 4 ] = value;
        } else {
            Module.HEAPU8[ address + 12 ] = 3;
            Module.HEAPF64[ address / 8 ] = value;
        }
    } else if( value === null ) {
        Module.HEAPU8[ address + 12 ] = 1;
    } else if( value === undefined ) {
        Module.HEAPU8[ address + 12 ] = 0;
    } else if( value === false ) {
        Module.HEAPU8[ address + 12 ] = 5;
    } else if( value === true ) {
        Module.HEAPU8[ address + 12 ] = 6;
    } else if( kind === "[object Symbol]" ) {
        var id = Module.STDWEB_PRIVATE.register_raw_value( value );
        Module.HEAPU8[ address + 12 ] = 15;
        Module.HEAP32[ address / 4 ] = id;
    } else {
        var refid = Module.STDWEB_PRIVATE.acquire_rust_reference( value );
        Module.HEAPU8[ address + 12 ] = 9;
        Module.HEAP32[ address / 4 ] = refid;
    }
};

// New browsers and recent Node
var cachedDecoder = ( typeof TextDecoder === "function"
    ? new TextDecoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextDecoder === "function"
        ? new util.TextDecoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedDecoder != null ) {
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        return cachedDecoder.decode( Module.HEAPU8.subarray( index, index + length ) );
    };

} else {
    // This is ported from Rust's stdlib; it's faster than
    // the string conversion from Emscripten.
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        var HEAPU8 = Module.HEAPU8;
        index = index|0;
        length = length|0;
        var end = (index|0) + (length|0);
        var output = "";
        while( index < end ) {
            var x = HEAPU8[ index++ ];
            if( x < 128 ) {
                output += String.fromCharCode( x );
                continue;
            }
            var init = (x & (0x7F >> 2));
            var y = 0;
            if( index < end ) {
                y = HEAPU8[ index++ ];
            }
            var ch = (init << 6) | (y & 63);
            if( x >= 0xE0 ) {
                var z = 0;
                if( index < end ) {
                    z = HEAPU8[ index++ ];
                }
                var y_z = ((y & 63) << 6) | (z & 63);
                ch = init << 12 | y_z;
                if( x >= 0xF0 ) {
                    var w = 0;
                    if( index < end ) {
                        w = HEAPU8[ index++ ];
                    }
                    ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));

                    output += String.fromCharCode( 0xD7C0 + (ch >> 10) );
                    ch = 0xDC00 + (ch & 0x3FF);
                }
            }
            output += String.fromCharCode( ch );
            continue;
        }
        return output;
    };
}

Module.STDWEB_PRIVATE.id_to_ref_map = {};
Module.STDWEB_PRIVATE.id_to_refcount_map = {};
Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap();
// Not all types can be stored in a WeakMap
Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map();
Module.STDWEB_PRIVATE.last_refid = 1;

Module.STDWEB_PRIVATE.id_to_raw_value_map = {};
Module.STDWEB_PRIVATE.last_raw_value_id = 1;

Module.STDWEB_PRIVATE.acquire_rust_reference = function( reference ) {
    if( reference === undefined || reference === null ) {
        return 0;
    }

    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
    var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;
    var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;

    var refid = ref_to_id_map.get( reference );
    if( refid === undefined ) {
        refid = ref_to_id_map_fallback.get( reference );
    }
    if( refid === undefined ) {
        refid = Module.STDWEB_PRIVATE.last_refid++;
        try {
            ref_to_id_map.set( reference, refid );
        } catch (e) {
            ref_to_id_map_fallback.set( reference, refid );
        }
    }

    if( refid in id_to_ref_map ) {
        id_to_refcount_map[ refid ]++;
    } else {
        id_to_ref_map[ refid ] = reference;
        id_to_refcount_map[ refid ] = 1;
    }

    return refid;
};

Module.STDWEB_PRIVATE.acquire_js_reference = function( refid ) {
    return Module.STDWEB_PRIVATE.id_to_ref_map[ refid ];
};

Module.STDWEB_PRIVATE.increment_refcount = function( refid ) {
    Module.STDWEB_PRIVATE.id_to_refcount_map[ refid ]++;
};

Module.STDWEB_PRIVATE.decrement_refcount = function( refid ) {
    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    if( 0 == --id_to_refcount_map[ refid ] ) {
        var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
        var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
        var reference = id_to_ref_map[ refid ];
        delete id_to_ref_map[ refid ];
        delete id_to_refcount_map[ refid ];
        ref_to_id_map_fallback.delete(reference);
    }
};

Module.STDWEB_PRIVATE.register_raw_value = function( value ) {
    var id = Module.STDWEB_PRIVATE.last_raw_value_id++;
    Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ] = value;
    return id;
};

Module.STDWEB_PRIVATE.unregister_raw_value = function( id ) {
    delete Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.get_raw_value = function( id ) {
    return Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.alloc = function alloc( size ) {
    return Module.web_malloc( size );
};

Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {
    return Module.web_table.get( ptr ).apply( null, args );
};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {
    var len = 0;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            ++len;
        } else if( u <= 0x7FF ) {
            len += 2;
        } else if( u <= 0xFFFF ) {
            len += 3;
        } else if( u <= 0x1FFFFF ) {
            len += 4;
        } else if( u <= 0x3FFFFFF ) {
            len += 5;
        } else {
            len += 6;
        }
    }
    return len;
};

Module.STDWEB_PRIVATE.prepare_any_arg = function( value ) {
    var arg = Module.STDWEB_PRIVATE.alloc( 16 );
    Module.STDWEB_PRIVATE.from_js( arg, value );
    return arg;
};

Module.STDWEB_PRIVATE.acquire_tmp = function( dummy ) {
    var value = Module.STDWEB_PRIVATE.tmp;
    Module.STDWEB_PRIVATE.tmp = null;
    return value;
};



    var HEAP8 = null;
    var HEAP16 = null;
    var HEAP32 = null;
    var HEAPU8 = null;
    var HEAPU16 = null;
    var HEAPU32 = null;
    var HEAPF32 = null;
    var HEAPF64 = null;

    Object.defineProperty( Module, 'exports', { value: {} } );

    function __web_on_grow() {
        var buffer = Module.instance.exports.memory.buffer;
        HEAP8 = new Int8Array( buffer );
        HEAP16 = new Int16Array( buffer );
        HEAP32 = new Int32Array( buffer );
        HEAPU8 = new Uint8Array( buffer );
        HEAPU16 = new Uint16Array( buffer );
        HEAPU32 = new Uint32Array( buffer );
        HEAPF32 = new Float32Array( buffer );
        HEAPF64 = new Float64Array( buffer );
        Module.HEAP8 = HEAP8;
        Module.HEAP16 = HEAP16;
        Module.HEAP32 = HEAP32;
        Module.HEAPU8 = HEAPU8;
        Module.HEAPU16 = HEAPU16;
        Module.HEAPU32 = HEAPU32;
        Module.HEAPF32 = HEAPF32;
        Module.HEAPF64 = HEAPF64;
    }

    return {
        imports: {
            env: {
                "__cargo_web_snippet_0326c402a09df06eb668ad14bff6431717298466": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);console.log(($0))
            },
            "__cargo_web_snippet_0435e43fde347d43075939eae13aa372fb9601ce": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).beginPath();
            },
            "__cargo_web_snippet_0f503de1d61309643e0e13a7871406891e3691c9": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return window;})());
            },
            "__cargo_web_snippet_19f4f60c027d536cca17ded92d03a0383e724f4e": function() {
                console.log("WOW X WINS");
            },
            "__cargo_web_snippet_1d3f576eb597a51bb802c752d3cfc29389445412": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).textAlign=($1);
            },
            "__cargo_web_snippet_1e65287b40ff2503a5bd21bba8369d5759ddb2d4": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).height=($1);
            },
            "__cargo_web_snippet_201decd68ca636d00b5b68ef0e37058531d0e122": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).strokeText(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_2566a94332f60241405fa45e6a379e8daa71af81": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof HTMLCanvasElement);
            },
            "__cargo_web_snippet_275c52510376b526efc3b77789bb01b8a440efd4": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).width;})());
            },
            "__cargo_web_snippet_2fa83e67fe53ac759a38940c46004f3eff4d97e9": function() {
                console.log("WOW O WINS");
            },
            "__cargo_web_snippet_31b100433af2dcbf517483ee42ffa60b64cdfeea": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof DOMRect);
            },
            "__cargo_web_snippet_32f73e1a10ada3bafa29a60084c3c7200043017d": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).top;})());
            },
            "__cargo_web_snippet_38b2cbaae3435ff623d80bcdd28833fc1a892195": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).pageYOffset;})());
            },
            "__cargo_web_snippet_49ae24e0f2d690c290030200ef793256363af281": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getContext("2d");})());
            },
            "__cargo_web_snippet_4df480b0c36ad03f462d9bfa2919bb2d404b7eaf": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).pageXOffset;})());
            },
            "__cargo_web_snippet_510e047dac02a657e1a4fcfc13b2498dedde5a1b": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);console.log("Returning with ignore = ",($0),", x and y and m = ",($1),($2),($3));
            },
            "__cargo_web_snippet_550049fd354fc2e2d13b8dacf6ebece3de7d60a0": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof CanvasRenderingContext2D);
            },
            "__cargo_web_snippet_64655a1289b56bca2d9a5c95e101442bd0d36acc": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).lineTo(($1),($2));
            },
            "__cargo_web_snippet_6c1f25bf7c9104accb489618515bd1869f4ca315": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).clientX;})());
            },
            "__cargo_web_snippet_6fcce0aae651e2d748e085ff1f800f87625ff8c8": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return document;})());
            },
            "__cargo_web_snippet_7003ece9a4ff4305e2022d0eabdf1d2301876bdd": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);console.log("Collapsing ",($0),($1),"with value ",($2));
            },
            "__cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723": function($0, $1, $2) {
                console.error( 'Panic location:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) + ':' + $2 );
            },
            "__cargo_web_snippet_7b0825ae89bed906bbdd29f8ee2ceb22c4fef516": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).width=($1);
            },
            "__cargo_web_snippet_7dc531e8c65617ed9c576d995ff452da6631daf7": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);console.log("Height:",($0));console.log("Width:",($1));
            },
            "__cargo_web_snippet_7e43238d5dec71ba02e4e74b13114774361fc9a0": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getBoundingClientRect();})());
            },
            "__cargo_web_snippet_7e69871d2f0243bddcb8cffc809fd6fb5fb78697": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).fillStyle=($1);
            },
            "__cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0": function($0) {
                Module.STDWEB_PRIVATE.decrement_refcount( $0 );
            },
            "__cargo_web_snippet_8c8a0fd988218bf31fae8adc33f715997855bce8": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).font=($1);
            },
            "__cargo_web_snippet_90f020fc20631c35b56f9fc9cbb9c287a99d55be": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof MouseEvent && o.type === "mousedown");
            },
            "__cargo_web_snippet_947e3c71a436d2534560c3daba2b3a52e02ec6d0": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).clientY;})());
            },
            "__cargo_web_snippet_95a438dd50b96776fe4ed8a27cce4d670597f742": function($0, $1, $2, $3, $4, $5) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);console.log("recurse: x ",($0),", y ",($1),", m ",($2),", x_origin ",($3),", y_origin ",($4),", ignore_first ",($5));
            },
            "__cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6": function($0, $1) {
                console.error( 'Panic error message:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) );
            },
            "__cargo_web_snippet_99c4eefdc8d4cc724135163b8c8665a1f3de99e4": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var listener=($1);($2).addEventListener(($3),listener);return listener;})());
            },
            "__cargo_web_snippet_9f22d4ca7bc938409787341b7db181f8dd41e6df": function($0) {
                Module.STDWEB_PRIVATE.increment_refcount( $0 );
            },
            "__cargo_web_snippet_af879f7e9f6e3db499feff63c418b5e2a6c654ac": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).strokeStyle=($1);
            },
            "__cargo_web_snippet_b2f12f45d22efd090ad11c42910de6a690b26ff5": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).textBaseline=($1);
            },
            "__cargo_web_snippet_b6fbe111e441333398599f63dc09b26f8d172654": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).innerHeight;})());
            },
            "__cargo_web_snippet_beb1ef96dc78600c39cf4a2d30d33fb4fa78d6e9": function($0, $1, $2, $3, $4, $5, $6) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);$6 = Module.STDWEB_PRIVATE.to_js($6);($0).arc(($1),($2),($3),($4),($5),($6));
            },
            "__cargo_web_snippet_c7547984f785b7dfdc411e5e87bba0771edcbeab": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).stroke();
            },
            "__cargo_web_snippet_cf0debbfec441e126df5ec4b805a71e969f49a75": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).fillRect(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_d3336fefc8646aa17b501ca0d1fc23db2bfd8df2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).height;})());
            },
            "__cargo_web_snippet_d5e30f74cb752784e06bd97a37b1f89b6c3433a7": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getElementById(($2));})());
            },
            "__cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf": function() {
                console.error( 'Encountered a panic!' );
            },
            "__cargo_web_snippet_dc8e7bbf6495cc2abd8497d69efceb13d03d30d7": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).clearRect(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_ddc80937ea2a5c8e373deb8ed94ce85a5b26eef2": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);console.log("M IS: ",($0));
            },
            "__cargo_web_snippet_e12643b35b62b2609b34cb7532d50e3f51e9ed8e": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).left;})());
            },
            "__cargo_web_snippet_e71ee53a2a209f1cf0422ec575b0007310ca5f90": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).moveTo(($1),($2));
            },
            "__cargo_web_snippet_e8d99a56b637089bb692dc9a0419c6a0d76030c5": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);console.log("M_NEXT I IS: ",($0));
            },
            "__cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);Module.STDWEB_PRIVATE.unregister_raw_value(($0));
            },
            "__cargo_web_snippet_ea6ad9d8415e84119621f5aa2c86a39abc588b75": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).innerWidth;})());
            },
            "__cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0": function($0) {
                Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 );
            },
                "__web_on_grow": __web_on_grow
            }
        },
        initialize: function( instance ) {
            Object.defineProperty( Module, 'instance', { value: instance } );
            Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
            Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
            Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__indirect_function_table } );

            
            __web_on_grow();
            Module.instance.exports.main();

            return Module.exports;
        }
    };
}
 ));
}));
