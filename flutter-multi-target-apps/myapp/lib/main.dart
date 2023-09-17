/* references
https://github.com/zuvola/flutter_grid_button
*/

import 'package:flutter/material.dart';
import 'package:flutter_grid_button/flutter_grid_button.dart';
import 'dart:collection';
import 'dart:io';

// /show Platform;

void main() {
  runApp(MyApp());
}

class Dispatch {
  static Map<String, Function> router = {
    '1': () {
      print('1funct');
    },
    '2': () {
      print('2func');
    },
    '3': () {
      print('3func');
    },
    '4': () async {
      print('4func');
      var x = await Process.run('bash', [
        '-c',
        '"ls"',
      ]);
      print(x.stdout);
    },
    'topbutton': () {
      // print('???');
      print(Platform.operatingSystem);
      print(Platform.environment);
      print(Platform.localHostname);
      print(Platform.version);
      print(Platform.operatingSystemVersion);
    },
    'bottombutton': () {
      print('bottombutton');
      print(Platform.localHostname);
    },
  };
  static entry_point(dynamic val) {
    Dispatch.router[val]!();
  }
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    const textStyle = TextStyle(fontSize: 26);
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('MobileAppBrain'),
        ),
        body: Builder(builder: (context) {
          return Padding(
            padding: const EdgeInsets.all(18.0),
            child: GridButton(
              textStyle: textStyle,
              borderColor: Colors.grey[300],
              borderWidth: 2,
              onPressed: (dynamic val) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(val.toString()),
                    duration: Duration(milliseconds: 400),
                  ),
                );
                Dispatch.entry_point(val);
              },
              items: [
                [
                  GridButtonItem(
                    child: Icon(
                      Icons.all_inclusive,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: 'topbutton',
                    color: Colors.deepPurple,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                ],
                [
                  GridButtonItem(
                    child: Icon(
                      Icons.ac_unit,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: '1',
                    color: Colors.blueGrey,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                  GridButtonItem(
                    child: Icon(
                      Icons.star_border_outlined,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: '2',
                    color: Colors.yellow,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                ],
                [
                  GridButtonItem(
                    child: Icon(
                      Icons.bolt,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: '3',
                    color: Colors.pink,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                  GridButtonItem(
                    child: Icon(
                      Icons.cake,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: '4',
                    color: Colors.orange,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                ],
                [
                  GridButtonItem(
                    child: Icon(
                      Icons.warning_amber,
                      size: 50,
                    ),
                    textStyle: textStyle.copyWith(color: Colors.white),
                    value: 'bottombutton',
                    color: Colors.green,
                    shape: BorderSide(width: 4),
                    borderRadius: 30,
                  ),
                ],
              ],
            ),
          );
        }),
        drawer: Drawer(
          child: Text('yo'),
        ),
      ),
    );
  }
}
