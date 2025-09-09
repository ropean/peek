namespace HttpStatus
{
  partial class FrmMain
  {
    /// <summary>
    /// Required designer variable.
    /// </summary>
    System.ComponentModel.IContainer components = null;

    /// <summary>
    /// Clean up any resources being used.
    /// </summary>
    /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
    protected override void Dispose(bool disposing)
    {
      if (disposing && (components != null))
      {
        components.Dispose();
      }
      base.Dispose(disposing);
    }

    #region Windows Form Designer generated code

    /// <summary>
    /// Required method for Designer support - do not modify
    /// the contents of this method with the code editor.
    /// </summary>
    void InitializeComponent()
    {
      System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(FrmMain));
      this.txtResponse = new System.Windows.Forms.TextBox();
      this.topPanel = new System.Windows.Forms.FlowLayoutPanel();
      this.txtURL = new System.Windows.Forms.TextBox();
      this.chkSSL = new System.Windows.Forms.CheckBox();
      this.chkPost = new System.Windows.Forms.CheckBox();
      this.btnRequest = new System.Windows.Forms.Button();
      this.chkRedirect = new System.Windows.Forms.CheckBox();
      this.topPanel.SuspendLayout();
      this.SuspendLayout();
      // 
      // txtResponse
      // 
      this.txtResponse.BackColor = System.Drawing.SystemColors.Control;
      this.txtResponse.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
      this.txtResponse.Font = new System.Drawing.Font("Microsoft Sans Serif", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
      this.txtResponse.Location = new System.Drawing.Point(22, 68);
      this.txtResponse.Margin = new System.Windows.Forms.Padding(10);
      this.txtResponse.Multiline = true;
      this.txtResponse.Name = "txtResponse";
      this.txtResponse.ReadOnly = true;
      this.txtResponse.ScrollBars = System.Windows.Forms.ScrollBars.Vertical;
      this.txtResponse.Size = new System.Drawing.Size(600, 200);
      this.txtResponse.TabIndex = 5;
      // 
      // topPanel
      // 
      this.topPanel.BackColor = System.Drawing.Color.Transparent;
      this.topPanel.BackgroundImage = global::HttpStatus.Properties.Resources.top_backgroud;
      this.topPanel.Controls.Add(this.txtURL);
      this.topPanel.Controls.Add(this.chkSSL);
      this.topPanel.Controls.Add(this.chkPost);
      this.topPanel.Controls.Add(this.chkRedirect);
      this.topPanel.Controls.Add(this.btnRequest);
      this.topPanel.Dock = System.Windows.Forms.DockStyle.Top;
      this.topPanel.Location = new System.Drawing.Point(0, 0);
      this.topPanel.Name = "topPanel";
      this.topPanel.Padding = new System.Windows.Forms.Padding(20, 10, 20, 10);
      this.topPanel.Size = new System.Drawing.Size(944, 55);
      this.topPanel.TabIndex = 0;
      // 
      // txtURL
      // 
      this.txtURL.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
      this.txtURL.Font = new System.Drawing.Font("Microsoft YaHei", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
      this.txtURL.Location = new System.Drawing.Point(22, 12);
      this.txtURL.Margin = new System.Windows.Forms.Padding(2);
      this.txtURL.Name = "txtURL";
      this.txtURL.Size = new System.Drawing.Size(500, 29);
      this.txtURL.TabIndex = 1;
      this.txtURL.Text = "aceapp.dev";
      this.txtURL.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.TxtURL_KeyPress);
      // 
      // chkSSL
      // 
      this.chkSSL.Checked = true;
      this.chkSSL.CheckState = System.Windows.Forms.CheckState.Checked;
      this.chkSSL.Font = new System.Drawing.Font("Microsoft YaHei", 11.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
      this.chkSSL.Location = new System.Drawing.Point(534, 13);
      this.chkSSL.Margin = new System.Windows.Forms.Padding(10, 3, 3, 3);
      this.chkSSL.Name = "chkSSL";
      this.chkSSL.Size = new System.Drawing.Size(60, 30);
      this.chkSSL.TabIndex = 2;
      this.chkSSL.Text = "SSL";
      this.chkSSL.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
      this.chkSSL.UseVisualStyleBackColor = true;
      // 
      // chkPost
      // 
      this.chkPost.Font = new System.Drawing.Font("Microsoft YaHei", 11.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
      this.chkPost.Location = new System.Drawing.Point(607, 13);
      this.chkPost.Margin = new System.Windows.Forms.Padding(10, 3, 3, 3);
      this.chkPost.Name = "chkPost";
      this.chkPost.Size = new System.Drawing.Size(60, 30);
      this.chkPost.TabIndex = 3;
      this.chkPost.Text = "Post";
      this.chkPost.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
      this.chkPost.UseVisualStyleBackColor = true;
      // 
      // btnRequest
      // 
      this.btnRequest.Font = new System.Drawing.Font("Microsoft YaHei", 11.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
      this.btnRequest.ForeColor = System.Drawing.Color.DeepPink;
      this.btnRequest.Location = new System.Drawing.Point(785, 12);
      this.btnRequest.Margin = new System.Windows.Forms.Padding(2);
      this.btnRequest.Name = "btnRequest";
      this.btnRequest.Size = new System.Drawing.Size(100, 30);
      this.btnRequest.TabIndex = 4;
      this.btnRequest.Text = "Request";
      this.btnRequest.UseVisualStyleBackColor = true;
      this.btnRequest.Click += new System.EventHandler(this.BtnRequest_Click);
      // 
      // chkRedirect
      // 
      this.chkRedirect.Font = new System.Drawing.Font("Microsoft YaHei", 11.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
      this.chkRedirect.Location = new System.Drawing.Point(680, 13);
      this.chkRedirect.Margin = new System.Windows.Forms.Padding(10, 3, 3, 3);
      this.chkRedirect.Name = "chkRedirect";
      this.chkRedirect.Size = new System.Drawing.Size(100, 30);
      this.chkRedirect.TabIndex = 5;
      this.chkRedirect.Text = "Redirect";
      this.chkRedirect.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
      this.chkRedirect.UseVisualStyleBackColor = true;
      // 
      // FrmMain
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.ClientSize = new System.Drawing.Size(944, 561);
      this.Controls.Add(this.txtResponse);
      this.Controls.Add(this.topPanel);
      this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
      this.Margin = new System.Windows.Forms.Padding(2);
      this.MinimumSize = new System.Drawing.Size(960, 600);
      this.Name = "FrmMain";
      this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
      this.Text = "HttpStatus";
      this.Load += new System.EventHandler(this.FrmMain_ClientSizeChanged);
      this.ClientSizeChanged += new System.EventHandler(this.FrmMain_ClientSizeChanged);
      this.topPanel.ResumeLayout(false);
      this.topPanel.PerformLayout();
      this.ResumeLayout(false);
      this.PerformLayout();

    }

    #endregion
    System.Windows.Forms.TextBox txtURL;
    System.Windows.Forms.Button btnRequest;
    System.Windows.Forms.TextBox txtResponse;
    private System.Windows.Forms.CheckBox chkSSL;
    private System.Windows.Forms.FlowLayoutPanel topPanel;
    private System.Windows.Forms.CheckBox chkPost;
    private System.Windows.Forms.CheckBox chkRedirect;
  }
}

