(function() {var implementors = {};
implementors['redis_client'] = ["impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;&amp;'a mut <a class='struct' href='redis_client/commands/struct.RedisCommand.html' title='redis_client::commands::RedisCommand'>RedisCommand</a>&gt; for <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'a [</a><a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a><a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.slice.html'>]</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;&amp;'a mut <a class='struct' href='redis_client/commands/struct.RedisCommand.html' title='redis_client::commands::RedisCommand'>RedisCommand</a>&gt; for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html' title='std::io::error::Error'>Error</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/core/str/struct.Utf8Error.html' title='core::str::Utf8Error'>Utf8Error</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/core/num/struct.ParseIntError.html' title='core::num::ParseIntError'>ParseIntError</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/errors/enum.ParsingError.html' title='redis_client::errors::ParsingError'>ParsingError</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.RecvError.html' title='std::sync::mpsc::RecvError'>RecvError</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.SendError.html' title='std::sync::mpsc::SendError'>SendError</a>&lt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(<a class='enum' href='redis_client/types/enum.SenderType.html' title='redis_client::types::SenderType'>SenderType</a>, <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.u32.html'>u32</a>, <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a>&gt;)</a>&gt;&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/enum.TryRecvError.html' title='std::sync::mpsc::TryRecvError'>TryRecvError</a>&gt; for <a class='enum' href='redis_client/errors/enum.RedisError.html' title='redis_client::errors::RedisError'>RedisError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/results/enum.RedisResult.html' title='redis_client::results::RedisResult'>RedisResult</a>&gt; for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/results/enum.RedisResult.html' title='redis_client::results::RedisResult'>RedisResult</a>&gt; for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/results/enum.RedisResult.html' title='redis_client::results::RedisResult'>RedisResult</a>&gt; for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/results/enum.RedisResult.html' title='redis_client::results::RedisResult'>RedisResult</a>&gt; for <a class='struct' href='https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html' title='std::collections::hash::map::HashMap'>HashMap</a>&lt;<a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>, <a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='redis_client/results/enum.RedisResult.html' title='redis_client::results::RedisResult'>RedisResult</a>&gt; for <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.i64.html'>i64</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()