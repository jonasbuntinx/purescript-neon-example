module Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)

foreign import get :: Unit -> String

main :: Effect Unit
main = do
  log $ get unit
