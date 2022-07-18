job ('Build Project K API') {
    scm {
        git('https://evolving-software@dev.azure.com/evolving-software/Business%20Management%20and%20Growth%20Platform/_git/backend_server_api')
    }
    triggers {
        scm('H/5 * * * *')
    }
    steps {
       dockerBuildAndPublish {
        repositoryName('evolvingsoftware/business-management-api')
        tag('${GIT_REVISION,length=9')
        registryCredentials('docker')
        createFingerprints(false)
        skipDecorate()
       }
    }
}
