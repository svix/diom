plugins {
    `java-library`
    `maven-publish`
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.fasterxml.jackson.core:jackson-core:2.19.4")
    implementation("com.fasterxml.jackson.core:jackson-annotations:2.19.4")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.19.4")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.19.4")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.19.4")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.google.code.findbugs:jsr305:3.0.2") // provides javax.annotation

    compileOnly("org.projectlombok:lombok:1.18.42")
    annotationProcessor("org.projectlombok:lombok:1.18.42")

    testImplementation("org.junit.jupiter:junit-jupiter:5.12.1")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(8)
    }
}

tasks.named<Test>("test") {
    useJUnitPlatform()
}
