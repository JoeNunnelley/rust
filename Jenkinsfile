#!/usr/bin/env groovy

pipeline {
    options {
        timeout(time: 60, unit: 'MINUTES')
        ansiColor('xterm')
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
