module Example exposing (..)

import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Test exposing (..)


suite : Test
suite =
    describe "todo: tests"
        [ test "this is a placeholder" <|
            \() -> 1 |> Expect.equal 1
        ]
