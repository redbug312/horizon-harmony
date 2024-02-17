import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  TextEditingController myController = TextEditingController();
  List services = [
    (Icons.route, "捷運路線"),
    (Icons.schedule, "旅程時間"),
    (Icons.paid_outlined, "票價查詢"),
    (Icons.people_outlined, "動態資訊 2.0"),
    (Icons.confirmation_number_outlined, "我的票卡"),
    (Icons.local_activity_outlined, "優惠券"),
    (Icons.route_outlined, "捷運路線 2.0"),
    (Icons.navigation_outlined, "轉乘推薦"),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("台北捷運"),
        centerTitle: true,
        foregroundColor: Theme.of(context).colorScheme.onPrimary,
        systemOverlayStyle: const SystemUiOverlayStyle(statusBarBrightness: Brightness.dark),
        leadingWidth: 100,
        leading: Row(
          children: [
            TextButton.icon(
              icon: Icon(Icons.language, color: Theme.of(context).colorScheme.onPrimary),
              label: const Text("中文"),
              style: TextButton.styleFrom(foregroundColor: Theme.of(context).colorScheme.onPrimary),
              onPressed: () {},
            ),
          ],
        ),
        actions: [
          IconButton(
            icon: Icon(Icons.notifications, color: Theme.of(context).colorScheme.onPrimary),
            onPressed: () {},
          ),
          IconButton(
            icon: Icon(Icons.menu, color: Theme.of(context).colorScheme.onPrimary),
            onPressed: () {},
          ),
        ],
        flexibleSpace: Container(
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.centerLeft,
              end: Alignment.centerRight,
              colors: [Colors.lightBlue.shade900, Colors.teal.shade600],
            ),
          ),
        ),
        bottom: PreferredSize(
          preferredSize: const Size.fromHeight(40),
          child: Container(),
          // child: Text("您想要搭乘臺北捷運，但不知道如何選擇最佳的路線和時間嗎？您可以使用本網站的起迄站查詢功能，輸入您的出發站和目的站，就可以獲得最新的票價、路線和時間資訊。您也可以查看各線的時刻表，或者下載路網圖和App，讓您的捷運出行更加便捷。"),
        ),
      ),
      body: ListView(
        shrinkWrap: true,
        padding: const EdgeInsets.all(10),
        children: [
          InkWell(
            onTap: () {},
            child: Ink(
              height: 150,
              decoration: BoxDecoration(
                color: Colors.grey.shade300,
              ),
              child: Icon(
                Icons.hail,
                color: Colors.grey.shade500,
                size: 100,
              ),
            ),
          ),
          Row(
            children: [
              const SizedBox(width: 5),
              Icon(
                Icons.place,
                size: 25,
                color: Theme.of(context).colorScheme.secondary,
              ),
              const SizedBox(width: 10),
              Expanded(
                child: Text(
                  "使用者名稱 您好，今天想去哪？",
                  maxLines: 1,
                  softWrap: false,
                  overflow: TextOverflow.fade,
                  style: Theme.of(context).textTheme.titleMedium,
                ),
              ),
              IconButton(
                icon: Icon(Icons.refresh, color: Theme.of(context).colorScheme.secondary),
                onPressed: () {},
              ),
            ],
          ),
          SizedBox(
            height: 40,
            child: SearchAnchor(
              builder: (context, controller) => SearchBar(
                controller: controller,
                leading: const Icon(Icons.search),
                elevation: MaterialStateProperty.all(0),
                hintText: "選擇或輸入您的目的地站名",
                backgroundColor: MaterialStateProperty.all(
                  Colors.grey.shade100,
                ),
              ),
              suggestionsBuilder: (context, controller) => {},
            ),
          ),
          const SizedBox(height: 10),
          const Divider(),
          GridView.builder(
            shrinkWrap: true,
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(crossAxisCount: 4),
            itemCount: services.length,
            itemBuilder: (context, index) => InkWell(
              onTap: () {},
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    services[index].$1,
                    size: 35,
                    color: Theme.of(context).colorScheme.primary,
                  ),
                  Flexible(
                    child: Text(
                      services[index].$2,
                      maxLines: 1,
                      softWrap: false,
                      overflow: TextOverflow.fade,
                      style: Theme.of(context).textTheme.labelLarge,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const Divider(),
          Text("線上商城 | 熱銷產品", style: Theme.of(context).textTheme.titleMedium),
          SizedBox(
            height: 100,
            width: double.infinity,
            child: ListView.builder(
              scrollDirection: Axis.horizontal,
              itemCount: 30,
              itemBuilder: (context, index) => Card(
                clipBehavior: Clip.antiAlias,
                elevation: 0,
                child: InkWell(
                  onTap: () {},
                  child: Ink(
                    padding: const EdgeInsets.all(15),
                    height: 100,
                    width: 120,
                    decoration: BoxDecoration(
                      color: Colors.grey.shade300
                    ),
                    child: Icon(
                      Icons.redeem,
                      color: Colors.grey.shade500,
                      size: 50,
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
      bottomNavigationBar: BottomNavigationBar(
        type: BottomNavigationBarType.fixed,
        currentIndex: 2,
        items: const [
          BottomNavigationBarItem(
            icon: Icon(Icons.route),
            label: "捷運路線",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.timer),
            label: "到站時刻",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.home),
            label: "首頁",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.luggage),
            label: "旅遊趣",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.person),
            label: "我的帳戶",
          ),
        ],
      ),
    );
  }
}
