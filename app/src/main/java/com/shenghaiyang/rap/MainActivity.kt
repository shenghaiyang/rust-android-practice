package com.shenghaiyang.rap

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val textView = findViewById<TextView>(R.id.text)
        textView.text = stringFromJNI("Rust")
    }

    private external fun stringFromJNI(input: String): String

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }
}