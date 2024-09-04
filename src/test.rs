#[cfg(test)]
mod tests {
    #[test]
    fn four_containers() {
        // Arrange
        let ps = r##"
        {"Command":"\"docker-entrypoint.s…\"","CreatedAt":"2023-11-01 12:41:17 +0300 MSK","ID":"a744d4d275e3","Image":"task-tracker","Labels":"com.docker.compose.project=task-tracker,com.docker.compose.project.config_files=docker-compose.yml,com.docker.compose.project.working_dir=/home/gitlab-runner/builds/6gYim1w8/0/inv/infrastructure/task-tracker,com.docker.compose.service=task-tracker,com.docker.compose.version=1.29.2,com.docker.compose.config-hash=9a51222f085e7e6e3b7de0e17d498e64c81a60cf4a33a3a19a37199150ae2dd9,com.docker.compose.container-number=1,com.docker.compose.oneoff=False","LocalVolumes":"0","Mounts":"","Names":"task-tracker_task-tracker_1","Networks":"task-tracker_default","Ports":"0.0.0.0:10040-\u003e3000/tcp, :::10040-\u003e3000/tcp","RunningFor":"20 minutes ago","Size":"0B","State":"running","Status":"Up 20 minutes"}
        {"Command":"\"docker-entrypoint.s…\"","CreatedAt":"2023-10-31 16:04:10 +0300 MSK","ID":"289a81af20d8","Image":"project-manager","Labels":"com.docker.compose.oneoff=False,com.docker.compose.project=project-manager,com.docker.compose.project.config_files=docker-compose.yml,com.docker.compose.project.working_dir=/home/gitlab-runner/builds/HyX66yZF/0/inv/infrastructure/project-manager,com.docker.compose.service=project-manager,com.docker.compose.version=1.29.2,com.docker.compose.config-hash=8715bb0a01ef7f7b206f685cd2f47acebe39652d8d83e316fee51d99497c9ef7,com.docker.compose.container-number=1","LocalVolumes":"0","Mounts":"","Names":"project-manager_project-manager_1","Networks":"project-manager_default","Ports":"0.0.0.0:10030-\u003e3000/tcp, :::10030-\u003e3000/tcp","RunningFor":"21 hours ago","Size":"0B","State":"running","Status":"Up 21 hours"}
        {"Command":"\"docker-entrypoint.s…\"","CreatedAt":"2023-10-31 15:24:36 +0300 MSK","ID":"854555d6f371","Image":"employees-manager","Labels":"com.docker.compose.project.config_files=docker-compose.yml,com.docker.compose.project.working_dir=/home/gitlab-runner/builds/yvLxBPb5/0/inv/infrastructure/employees-manager,com.docker.compose.service=employees-manager,com.docker.compose.version=1.29.2,com.docker.compose.config-hash=be4a86c18f334514c3d03cd86f3f508a9896d1f56997e0822da4488c495d2e21,com.docker.compose.container-number=1,com.docker.compose.oneoff=False,com.docker.compose.project=employees-manager","LocalVolumes":"0","Mounts":"","Names":"employees-manager_employees-manager_1","Networks":"employees-manager_default","Ports":"0.0.0.0:10010-\u003e3000/tcp, :::10010-\u003e3000/tcp","RunningFor":"22 hours ago","Size":"0B","State":"running","Status":"Up 22 hours"}
        {"Command":"\"docker-entrypoint.s…\"","CreatedAt":"2023-10-27 18:15:47 +0300 MSK","ID":"52fc47095c2d","Image":"unidash","Labels":"com.docker.compose.container-number=1,com.docker.compose.oneoff=False,com.docker.compose.project=client,com.docker.compose.project.config_files=client/docker-compose.yml,com.docker.compose.project.working_dir=/home/gitlab-runner/builds/qg_JRYty/0/inv/sng/unidash/client,com.docker.compose.service=unidash,com.docker.compose.version=1.29.2,com.docker.compose.config-hash=378c39ac06e7f1d90c4ceed926f8303d13913c832b344cbcc08f09ddb458fbc5","LocalVolumes":"0","Mounts":"","Names":"client_unidash_1","Networks":"client_default","Ports":"0.0.0.0:10020-\u003e8080/tcp, :::10020-\u003e8080/tcp","RunningFor":"4 days ago","Size":"0B","State":"running","Status":"Up 4 days"}
        "##;

        // Act
        // TODO: Execute the code under test.

        // raise an error
        // panic!("Make this test fail!");

        // Assert
        // TODO: Check the result(s).
        println!("Hello, world!");
    }

    #[test]
    fn test_case_name_ok() {
        // Arrange
        // TODO: Set up any necessary data or state.

        // Act
        // TODO: Execute the code under test.

        // raise an error
        // panic!("Make this test fail!");

        let a = 12;

        // Assert
        assert_eq!(a, 12);
        // TODO: Check the result(s).
        println!("Hello, world!");
    }
}
