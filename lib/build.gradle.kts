plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.rust.android)
    alias(libs.plugins.maven.publish)
    idea
}

android {
    namespace = "moe.lava.sunflower"
    ndkVersion = "29.0.14206865"
    compileSdk = 36

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    kotlinOptions {
        jvmTarget = "11"
    }
}

cargo {
    module = "./src/main/rust"
    libname = "sunflower"
    targets = listOf("arm64", "arm", "x86", "x86_64")
    profile = "release"
}

dependencies {
    api("net.java.dev.jna:jna:5.18.1@aar")
    implementation(libs.androidx.annotation.jvm)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
}

publishing {
    repositories {
        mavenLocal()
    }
}

mavenPublishing {
    coordinates("moe.lava", "sunflower", "0.1.0")

    pom {
        name = "Sunflower"
        description = "UniFFI bindings for davey, an implementation of Discord's E2EE Protocol DAVE"
        inceptionYear = "2025"
        url = "https://github.com/LavaDesu/Sunflower"
        licenses {
            license {
                name = "MIT Licence"
                url.set("https://github.com/LavaDesu/Sunflower/blob/master/LICENCE")
            }
        }
        developers {
            developer {
                id = "lavadesu"
                name = "Cilly Leang"
                url = "https://github.com/LavaDesu"
            }
        }

        scm {
            url = "https://github.com/LavaDesu/Sunflower"
            connection = "scm:git:git://github.com/LavaDesu/Sunflower.git"
            developerConnection = "scm:git:ssh://git@github.com/LavaDesu/Sunflower.git"
        }
    }
}