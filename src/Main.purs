module Main where

import Prelude

import Control.Monad.Reader (ReaderT, ask, asks, runReaderT)
import Effect (Effect)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Uncurried (EffectFn1, runEffectFn1)

foreign import data App :: Type

foreign import mkApp :: Effect App

foreign import run_ :: EffectFn1 App Unit

foreign import addPlugins_ :: EffectFn1 App Unit

foreign import addStartupSystem_ :: EffectFn1 App Unit

run :: App -> Effect Unit
run = runEffectFn1 run_

addPlugins :: App -> Effect Unit
addPlugins = runEffectFn1 addPlugins_

addStartupSystem :: App -> Effect Unit
addStartupSystem = runEffectFn1 addStartupSystem_

newtype AppM a = AppM (ReaderT App Effect a)

runAppM :: AppM ~> Effect
runAppM (AppM m) = do 
  app <- mkApp 
  runReaderT m app



derive newtype instance Functor AppM

derive newtype instance Apply AppM

derive newtype instance Applicative AppM

derive newtype instance Bind AppM

derive newtype instance Monad AppM

derive newtype instance MonadEffect AppM

main :: Effect Unit
main = runAppM $ AppM do 
    app <- ask
    liftEffect $ addPlugins app 
    liftEffect $ addStartupSystem app 
    liftEffect $ run app