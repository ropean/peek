using System;
using System.Net;
using System.Net.Security;
using System.Security.Cryptography.X509Certificates;
using System.Windows.Forms;

namespace HttpStatus
{
  static class Program
  {
    /// <summary>
    /// The main entry point for the application.
    /// </summary>
    [STAThread]
    static void Main()
    {
      Application.EnableVisualStyles();

      Application.SetCompatibleTextRenderingDefault(false);

      // Keep cross-thread checks enabled; UI updates are done on the UI thread via async/await

      ServicePointManager.SecurityProtocol = SecurityProtocolType.Ssl3
        | SecurityProtocolType.Tls
        | SecurityProtocolType.Tls11
        | SecurityProtocolType.Tls12;

      ServicePointManager.ServerCertificateValidationCallback = ValidateServerCertificate;

      Application.Run(new FrmMain());
    }

    static bool ValidateServerCertificate(object sender, X509Certificate certificate, X509Chain chain, SslPolicyErrors sslPolicyErrors)
    {
      return true;
    }
  }
}
