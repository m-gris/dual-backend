ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / scalaVersion := "3.2.1"

// Matching dependency versions from typelevel-rite-of-passage
lazy val catsEffectVersion = "3.3.14"
lazy val http4sVersion     = "0.23.15"
lazy val slf4jVersion      = "2.0.0"

// The trick: use "." as the project root but customize source directories
// This makes `sbt run` work directly without needing `root/run`
lazy val root = (project in file("."))
  .settings(
    name := "zero2prod-scala",
    organization := "com.example",
    // Point to scala-version subdirectory for sources
    Compile / scalaSource := baseDirectory.value / "scala-version" / "src" / "main" / "scala",
    Test / scalaSource := baseDirectory.value / "scala-version" / "src" / "test" / "scala",
    // Target directory also goes into scala-version to keep it clean
    target := baseDirectory.value / "scala-version" / "target",
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
