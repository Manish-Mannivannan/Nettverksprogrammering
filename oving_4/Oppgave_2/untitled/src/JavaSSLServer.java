import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.logging.Level;
import java.util.logging.Logger;
import javax.net.ssl.SSLServerSocketFactory;

/**
 * @web http://java-buddy.blogspot.com/
 */
public class JavaSSLServer {

    static final int port = 8000;

    public static void main(String[] args) {

        System.setProperty("javax.net.ssl.keyStore", "src/examplestore");
        System.setProperty("javax.net.ssl.keyStorePassword", "password");


        SSLServerSocketFactory sslServerSocketFactory =
                (SSLServerSocketFactory)SSLServerSocketFactory.getDefault();

        try {
            ServerSocket sslServerSocket =
                    sslServerSocketFactory.createServerSocket(port);
            System.out.println("SSL ServerSocket started");
            System.out.println(sslServerSocket.toString());

            Socket socket = sslServerSocket.accept();
            System.out.println("ServerSocket accepted");

            PrintWriter out = new PrintWriter(socket.getOutputStream(), true);
            try (BufferedReader bufferedReader =
                         new BufferedReader(
                                 new InputStreamReader(socket.getInputStream()))) {
                String line;
                while((line = bufferedReader.readLine()) != null){
                    System.out.println(line);
                    out.println(line);
                }
            }
            System.out.println("Closed");

        } catch (IOException ex) {
            Logger.getLogger(JavaSSLServer.class.getName())
                    .log(Level.SEVERE, null, ex);
        }
    }

}
