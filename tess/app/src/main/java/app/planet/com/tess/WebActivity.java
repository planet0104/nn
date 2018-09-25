package app.planet.com.tess;

import android.app.Activity;
import android.os.Bundle;
import android.support.annotation.Nullable;
import android.webkit.WebView;

import com.mozilla.greetings.RustGreetings;

public class WebActivity extends Activity {
    private WebView webView;
    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        webView = new WebView(this);
        setContentView(webView);
        new Thread(new Runnable() {
            @Override
            public void run() {
                RustGreetings.serve();
            }
        }).start();
        webView.postDelayed(new Runnable() {
            @Override
            public void run() {
                webView.loadUrl("http://127.0.0.1:8000");
            }
        }, 3000);
    }
}
