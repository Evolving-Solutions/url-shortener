job ("Build Project K API") {
    scm {
        git('git@ssh.dev.azure.com:v3/evolving-software/Business%20Management%20and%20Growth%20Platform/backend_server_api') 
    }
    triggers {
        scm('H/5 * * * *')

    }
    wrappers{
        docker('docker')
    }
    steps {
       dockerBuildAndPublish {
        repositoryName('evolvingsoftware/projectk-api')
        tag('${GIT_REVISION,length=9')
        registryCredentials('docker')
        forcePull('false')
        forceTag('false')
        createFingerprints(false)
        skipDecorate()
       }
    }
}