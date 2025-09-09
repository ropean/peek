using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Drawing;

namespace HttpStatus
{
  public partial class FrmMain : Form
  {
    public FrmMain()
    {
      InitializeComponent();
    }

    async void BtnRequest_Click(object sender, EventArgs e)
    {
      await DoRequestAsync();
    }

    async Task DoRequestAsync()
    {
      try
      {
        string url = ParseURI();

        txtResponse.Text = "Requesting...";

        txtResponse.Text = $"Your requested address is: {url}" + Environment.NewLine;

        var clientIPs = ParseAddressList(GetIPList(Environment.MachineName).ToList());

        txtResponse.Text += "Your IP(s):" + clientIPs + Environment.NewLine + Environment.NewLine;

        List<IPAddress> addressList = new List<IPAddress>();

        if (IPAddress.TryParse(url.Replace(HTTP_Scheme, "").Replace(HTTPS_Scheme, ""), out IPAddress address))
        {
          addressList.Add(address);
        }
        else
        {
          addressList.AddRange(GetIPList(new Uri(url).Host));
        }

        txtResponse.Text += "Responded IP(s):" + ParseAddressList(addressList) + Environment.NewLine;

        var handler = new HttpClientHandler()
        {
          AllowAutoRedirect = chkRedirect.Checked,
          UseCookies = false,
        };

        using (var client = new HttpClient(handler))
        {
          client.Timeout = TimeSpan.FromSeconds(10000);
          client.DefaultRequestHeaders.CacheControl = new CacheControlHeaderValue { NoCache = true };

          HttpResponseMessage responseMessage;
          if (chkPost.Checked)
          {
            responseMessage = await client.PostAsync(url, new StringContent(string.Empty));
          }
          else
          {
            responseMessage = await client.GetAsync(url);
          }

          var responseContent = await responseMessage.Content.ReadAsStringAsync();

          List<string> lines = new List<string>
          {
            Environment.NewLine,
            $"Responded status code: {(int)responseMessage.StatusCode}",
            Environment.NewLine,
            "Responded headers:",
          };

          lines.AddRange(responseMessage.Headers.Select(d => $"{d.Key}: {string.Join(", ", d.Value)}"));
          lines.AddRange(responseMessage.Content.Headers.Select(d => $"{d.Key}: {string.Join(", ", d.Value)}"));

          lines.Add(Environment.NewLine);

          lines.Add("Responded source code:");

          lines.Add(responseContent);
          txtResponse.Text += string.Join(Environment.NewLine, lines);
        }
      }
      catch (Exception exc)
      {
        MessageBox.Show(exc.Message);
      }
    }

    public const string HTTP_Scheme = "http://";
    public const string HTTPS_Scheme = "https://";

    string ParseURI()
    {
      var url = txtURL.Text.Trim().ToLower();

      if (chkSSL.Checked)
      {
        if (url.StartsWith(HTTP_Scheme))
        {
          url = url.Replace(HTTP_Scheme, HTTPS_Scheme);
        }
        else if (url.StartsWith(HTTPS_Scheme))
        {

        }
        else
        {
          url = HTTPS_Scheme + url;
        }
      }
      else
      {
        if (url.StartsWith(HTTP_Scheme))
        {

        }
        else if (url.StartsWith(HTTPS_Scheme))
        {

        }
        else
        {
          url = HTTP_Scheme + url;
        }
      }

      return url;
    }

    public static IPAddress[] GetIPList(string hostNameOrAddress)
    {
      IPHostEntry hostEntry = Dns.GetHostEntry(hostNameOrAddress);

      return hostEntry.AddressList;
    }

    string ParseAddressList(List<IPAddress> addressList)
    {
      return (addressList.Count > 1 ? Environment.NewLine : " ") + string.Join(Environment.NewLine, addressList);
    }

    void TxtURL_KeyPress(object sender, KeyPressEventArgs e)
    {
      if (e.KeyChar == '\r')
      {
        BtnRequest_Click(null, null);
      }
    }

    private void FrmMain_ClientSizeChanged(object sender, EventArgs e)
    {
      txtResponse.Location = new Point(10, topPanel.Height + 10);
      txtResponse.Width = ClientSize.Width - 50;
      txtResponse.Height = ClientSize.Height - topPanel.Height - 60;
    }
  }
}