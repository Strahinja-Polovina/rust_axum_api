pipeline {
    agent any

     stages {
            stage('Build') {
                steps {
                        sh 'cargo build --release'
                    
                }
            }

            stage('Test') {
                steps {
                        sh 'cargo test'
                    }
                }
            }

            stage('Deploy') {
                steps {
                        sh 'cargo publish'
                    }
                }
            }
        }
}
