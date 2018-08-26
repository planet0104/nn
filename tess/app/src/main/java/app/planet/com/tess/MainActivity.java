package app.planet.com.tess;

import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.util.Pair;

import com.googlecode.leptonica.android.Pix;
import com.googlecode.leptonica.android.Pixa;
import com.googlecode.tesseract.android.ResultIterator;
import com.googlecode.tesseract.android.TessBaseAPI;

import java.io.File;
import java.io.IOException;
import java.util.Iterator;
import java.util.List;

public class MainActivity extends AppCompatActivity {
    static final String TAG = MainActivity.class.getSimpleName();

    /*

    1、整体汉字打分用 tesseract 评分。
        笔画全部正确写完以后，对view进行截图并使用tesseract识别，打分。如果无法识别对应的文字，打0分不予通过。
    2、每一个笔画用cnn进行粗略判断，防止乱写。
       根据常用汉字笔画表，训练全部的笔画，相近无法区分的笔画归为一类。
       用户每写完一笔，进行笔画识别。

     */

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        //将tessdata文件夹解压到files文件夹
        boolean ok = false;
        try {
            File tessdataDir = new File(getFilesDir(), "tessdata");
            if(!tessdataDir.exists()){
                if(FileUtils.unpackZip(getAssets().open("tessdata.zip"), getFilesDir())){
                    Log.d(TAG, "tessdata解压成功");
                    ok = true;
                }else{
                    Log.e(TAG, "tessdata解压失败");
                }
            }else{
                Log.e(TAG, "tessdata已经存在");
                ok = true;
            }
        } catch (IOException e) {
            e.printStackTrace();
            Log.e(TAG, "tessdata文件夹读取失败!");
        }

        if(ok){
            try {
                Bitmap bitmap = BitmapFactory.decodeStream(getAssets().open("img2.png"));
                TessBaseAPI tessBaseAPI = new TessBaseAPI();
                Log.d(TAG, "版本:"+tessBaseAPI.getVersion());
                tessBaseAPI.init(getFilesDir().getAbsolutePath(), "chi_sim");//参数后面有说明。
                tessBaseAPI.setImage(bitmap);
                String text = tessBaseAPI.getUTF8Text();
                ResultIterator resultIterator = tessBaseAPI.getResultIterator();
                int level = TessBaseAPI.PageIteratorLevel.RIL_SYMBOL;
                do{
                    Log.d(TAG, resultIterator.getUTF8Text(level)+"-"+resultIterator.confidence(level));
                }while(resultIterator.next(level));

                resultIterator.delete();

                Log.d(TAG, "识别结果:"+text);
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}
