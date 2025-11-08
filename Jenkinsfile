pipeline {
    agent any
    
    stages {
        stage('Build') {
            steps {
                echo "Building..."
            }
        }
        
        stage('Test') {
            steps {
                echo "Running tests..."
            }
        }

        stage('a') {
            steps {
                cargo run
              }
          }
        
        stage('Deploy') {
            steps {
                echo "Deploying..."
            }
        }
    }
}
