#!/usr/bin/env groovy

pipeline {
    options {
        timeout(time: 60, unit: 'MINUTES')
        ansiColor('xterm')
    }
    environment {
        registry = "joenunnelley/docker-test"
        registryCredential = "dockerhub"
    }

    agent any

    stages {
        stage('Echo') {
            steps {
                sh 'echo'
             }
        }
    }
}
