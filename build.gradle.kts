//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

plugins {
	id("sandpolis-instance")
	id("sandpolis-publish")
}

dependencies {
	proto("com.sandpolis:core.foundation:+:rust@zip")
	proto("com.sandpolis:core.instance:+:rust@zip")
	proto("com.sandpolis:core.net:+:rust@zip")
	proto("com.sandpolis:plugin.snapshot:+:rust@zip")
}

val buildAmd64 by tasks.creating(Exec::class) {
	dependsOn("extractDownloadedProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cargo", "+nightly", "build", "--release", "--target=x86_64-unknown-uefi"))
	outputs.files("target/x86_64-unknown-uefi/release/agent")
}

val buildAarch64 by tasks.creating(Exec::class) {
	dependsOn("extractDownloadedProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cargo", "+nightly", "build", "--release", "--target=aarch64-unknown-uefi"))
	outputs.files("target/aarch64-unknown-uefi/release/agent")
}

tasks.findByName("build")?.dependsOn(buildAmd64, buildAarch64)

publishing {
	publications {
		create<MavenPublication>("agent") {
			groupId = "com.sandpolis"
			artifactId = "agent.boot"
			version = project.version.toString()

			artifact(buildAmd64.outputs.files.filter { it.name == "agent" }.getSingleFile()) {
				classifier = "amd64"
			}

			artifact(buildAarch64.outputs.files.filter { it.name == "agent" }.getSingleFile()) {
				classifier = "aarch64"
			}
		}
		tasks.findByName("publishAgentPublicationToCentralStagingRepository")?.dependsOn("build")
	}
}
