{ name = "purescript-neon-example"
, dependencies = 
  [ "console"
  , "effect"
  , "prelude"
  , "transformers"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs", "test/**/*.purs" ]
}
