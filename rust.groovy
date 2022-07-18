job ('Build Project K API') {
    scm {
        git('https://evolving-software@dev.azure.com/evolving-software/Business%20Management%20and%20Growth%20Platform/_git/backend_server_api') { node -> // is hudson.plugins.git.GitSCM
            
            node / gitConfigName('riley')
        node / gitConfigEmail('riley@evolvingsoftware.io')}
    }
    triggers {
        scm('H/5 * * * *')
    }
    steps {
       dockerBuildAndPublish {
        repositoryName('evolvingsoftware/business-management-api')
        tag('${GIT_REVISION,length=9')
        registryCredentials('4e97f946-e36e-4299-b368-28dddb01539b')
        forcePull(false)
        forceTag(false)
        createFingerprints(false)
        skipDecorate()
       }
    }
}
