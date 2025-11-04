import com.android.build.gradle.internal.errors.MessageReceiverImpl
import com.android.build.gradle.options.SyncOptions
import com.android.build.gradle.tasks.BundleAar
import com.android.builder.dexing.ClassFileEntry
import com.android.builder.dexing.ClassFileInput
import com.android.builder.dexing.DexArchiveBuilder
import com.android.builder.dexing.DexParameters
import com.android.builder.dexing.r8.ClassFileProviderFactory
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.slf4j.LoggerFactory
import java.nio.file.Path
import java.nio.file.Paths
import java.util.function.BiPredicate
import java.util.stream.Stream

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

tasks.register("buildDexRelease") {
    outputs.dir(layout.buildDirectory.dir("intermediates/dex/"))

    val compileTask = project.tasks.getByName("compileReleaseKotlin") as KotlinCompile
    dependsOn(compileTask)
    inputs.dir(compileTask.destinationDirectory.asFile.get())

    doLast {
        val bootClasspath = ClassFileProviderFactory(android.bootClasspath.map(File::toPath))
        val classpath = ClassFileProviderFactory(listOf<Path>())
        val dexBuilder = DexArchiveBuilder.createD8DexBuilder(
            DexParameters(
                minSdkVersion = android.defaultConfig.minSdkVersion!!.apiLevel,
                debuggable = false,
                dexPerClass = false,
                withDesugaring = true,
                desugarBootclasspath = bootClasspath,
                desugarClasspath = classpath,
                coreLibDesugarConfig = null,
                enableApiModeling = false,
                messageReceiver = MessageReceiverImpl(
                    SyncOptions.ErrorFormatMode.HUMAN_READABLE,
                    LoggerFactory.getLogger("buildDexRelease")
                )
            )
        )

        outputs.files.singleFile.mkdirs()

        try {
            dexBuilder.convert(
                input = inputs.files
                    .filter(File::exists)
                    .filter { f -> f.extension == "class" }
                    .files.stream().map {
                        val bytes = it.readBytes()
                        MemoryClassFileEntry(it.name, bytes.size.toLong(), bytes)
                    },
                dexOutput = outputs.files.singleFile.toPath(),
                globalSyntheticsOutput = null,
            )
        } finally {
            bootClasspath.close()
            classpath.close()
        }
    }
}

afterEvaluate {
    tasks.named<BundleAar>("bundleReleaseAar") {
        val dexTask = tasks.named("buildDexRelease").get()
        dependsOn(dexTask)
        from(dexTask.outputs.files.singleFile)
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

class MemoryClassFileEntry(
    private val name: String,
    private val size: Long,
    private val bytes: ByteArray
) : ClassFileEntry {
    override fun name() = name
    override fun getSize() = size
    override fun getRelativePath() = ""
    override fun readAllBytes() = bytes
    override fun getInput() = object : ClassFileInput {
        override fun close() {}
        override fun entries(filter: BiPredicate<Path, String>?) = Stream.empty<ClassFileEntry>()

        override fun getPath() = Paths.get("")
    }

    override fun readAllBytes(bytes: ByteArray?): Int {
        bytes ?: return 0
        this.bytes.copyInto(bytes, 0, 0, this.bytes.lastIndex)
        return this.bytes.size
    }
}