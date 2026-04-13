plugins {
    `java-library`
    `maven-publish`
    signing
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
}

val GROUP: String by project
val VERSION_NAME: String by project
val POM_ARTIFACT_ID: String by project
val POM_NAME: String by project
val POM_DESCRIPTION: String by project
val POM_URL: String by project
val POM_SCM_URL: String by project
val POM_SCM_CONNECTION: String by project
val POM_SCM_DEV_CONNECTION: String by project
val POM_LICENCE_NAME: String by project
val POM_LICENCE_URL: String by project
val POM_LICENCE_DIST: String by project
val POM_DEVELOPER_ID: String by project
val POM_DEVELOPER_NAME: String by project
val POM_DEVELOPER_EMAIL: String by project
val POM_ORGANIZATION_URL: String by project

group = GROUP
version = VERSION_NAME

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.fasterxml.jackson.core:jackson-core:2.19.4")
    implementation("com.fasterxml.jackson.core:jackson-annotations:2.19.4")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.19.4")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.19.4")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.19.4")
    implementation("org.msgpack:jackson-dataformat-msgpack:0.9.11")
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
    withSourcesJar()
    withJavadocJar()
}

tasks.named<Test>("test") {
    useJUnitPlatform {
        excludeTags("integration")
    }
}

tasks.register<Test>("integrationTest") {
    useJUnitPlatform {
        includeTags("integration")
    }
}

nexusPublishing {
    packageGroup.set(GROUP)
    repositories {
        sonatype {
            nexusUrl.set(uri("https://ossrh-staging-api.central.sonatype.com/service/local/"))
            snapshotRepositoryUrl.set(uri("https://central.sonatype.com/repository/maven-snapshots/"))
            username.set(findProperty("NEXUS_USERNAME")?.toString() ?: "")
            password.set(findProperty("NEXUS_PASSWORD")?.toString() ?: "")
        }
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = GROUP
            artifactId = POM_ARTIFACT_ID
            version = VERSION_NAME

            from(components["java"])

            pom {
                name.set(POM_NAME)
                description.set(POM_DESCRIPTION)
                url.set(POM_URL)

                licenses {
                    license {
                        name.set(POM_LICENCE_NAME)
                        url.set(POM_LICENCE_URL)
                        distribution.set(POM_LICENCE_DIST)
                    }
                }
                developers {
                    developer {
                        id.set(POM_DEVELOPER_ID)
                        name.set(POM_DEVELOPER_NAME)
                        email.set(POM_DEVELOPER_EMAIL)
                    }
                }
                scm {
                    connection.set(POM_SCM_CONNECTION)
                    developerConnection.set(POM_SCM_DEV_CONNECTION)
                    url.set(POM_SCM_URL)
                }
                organization {
                    name.set(POM_DEVELOPER_NAME)
                    url.set(POM_ORGANIZATION_URL)
                }
            }
        }
    }
}

signing {
    isRequired = !version.toString().endsWith("-SNAPSHOT") && !project.hasProperty("skipSigning")
    useGpgCmd()
    sign(publishing.publications["maven"])
}
