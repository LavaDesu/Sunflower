plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.rust.android)
    idea
}

android {
    namespace = "moe.lava.sunflower"
    compileSdk = 36
    ndkVersion = "29.0.14206865"

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildFeatures {
        prefabPublishing = true
    }

    prefab {
        create("davey")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
//    libraryVariants.all {
//        val variant = this
//        val out = File(layout.buildDirectory.get().asFile, "generated/source/uniffi/${variant.name}")
//        val task = tasks.register<Exec>("generate${variant.name.capitalize()}UniFFIBindings") {
//            workingDir = project.projectDir
//            commandLine(
//                "uniffi-bindgen", "generate",
//                "--library", "target/release/libsunflower.so",
//                "--language", "kotlin",
//                "--out-dir", out
//            )
//        }
//        variant.javaCompileProvider.get().dependsOn(task)
//        val sourceSet = variant.sourceSets.find { it.name == variant.name }
////        sourceSet?.javaDirectories?.add(out)
//        idea.module.generatedSourceDirs.add(File(out, "uniffi"))
//    }
    kotlinOptions {
        jvmTarget = "11"
    }
}

cargo {
    module = "./src/main/rust"
    libname = "sunflower"
    targets = listOf("arm64")
//    targets = listOf("arm64", "arm", "x86", "x86_64")
    profile = "release"
}

dependencies {
    api("net.java.dev.jna:jna:5.18.1@aar")
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
}