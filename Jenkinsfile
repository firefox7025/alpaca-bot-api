pipeline {
    agent {
        docker { image 'rust:1-slim' }
    }
    stages {
        stage('Build') {
            steps {
                sh "cargo build --release"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
    }
}