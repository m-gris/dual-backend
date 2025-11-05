ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / scalaVersion := "3.2.1"

// Matching dependency versions from typelevel-rite-of-passage
lazy val catsEffectVersion = "3.3.14"
lazy val http4sVersion     = "0.23.15"
lazy val slf4jVersion      = "2.0.0"

lazy val root = (project in file("."))
  .settings(
    name := "zero2prod-scala",
    organization := "com.example",
    // Point to scala-src instead of default src/main/scala
    Compile / scalaSource := baseDirectory.value / "scala-src" / "main" / "scala",
    // Tell sbt to watch scala-src for changes (needed for ~run to work)
    watchSources += WatchSource(
      baseDirectory.value / "scala-src",
      "*.scala" || "*.sbt"
    ),
    libraryDependencies ++= Seq(
      // Cats Effect - the async runtime (equivalent to tokio in Rust)
      "org.typelevel" %% "cats-effect" % catsEffectVersion,

      // Http4s - the web framework (equivalent to actix-web in Rust)
      "org.http4s" %% "http4s-dsl"          % http4sVersion,
      "org.http4s" %% "http4s-ember-server" % http4sVersion,

      // Logging (equivalent to Rust's println! for now, but more structured)
      "org.slf4j" % "slf4j-simple" % slf4jVersion
    )
  )
