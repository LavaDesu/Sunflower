package moe.lava.sunflower.sample

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import moe.lava.sunflower.DaveSession
import kotlin.random.Random
import kotlin.random.nextULong

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                Column(Modifier.padding(innerPadding)) {
                    var session by remember { mutableStateOf(DaveSession(1u, Random.nextULong(), Random.nextULong(), null)) }
                    Text(
                        session.toString()
                    )
                    Button(
                        onClick = { session = DaveSession(1u, Random.nextULong(), Random.nextULong(), null) }
                    ) {
                        Text("recycle")
                    }
                }
            }
        }
    }
}