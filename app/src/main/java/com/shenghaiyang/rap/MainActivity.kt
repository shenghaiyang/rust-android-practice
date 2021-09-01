package com.shenghaiyang.rap

import android.annotation.SuppressLint
import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    @SuppressLint("SetTextI18n")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val textView = findViewById<TextView>(R.id.text)
        textView.text = "${stringFromJNI("rust")}\n${dynamicRegisterMethod("rust")}"
    }

    private external fun stringFromJNI(input: String): String

    private external fun dynamicRegisterMethod(input: String): String

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }
}