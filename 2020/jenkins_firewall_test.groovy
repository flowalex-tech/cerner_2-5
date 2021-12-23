node('LINUX') {
// cerner_2^5_2020
// simple script to run commands after applying local deny all firewall and allowing certain connections through
    try {
        sh(returnStatus: true, script: "hostname -f")
        sh(returnStatus: true, script: "docker run hello-world")
        sh(returnStatus: true, script: "docker image rm hello-world")
        sh(returnStatus: true, script:  "wget https://raw.githubusercontent.com/flowalex-tech/scripts/mainline/os_utils/landsshkeys.sh && landsshkeys.sh    ")
    }

    finally {
        cleanWs()
    }
}
