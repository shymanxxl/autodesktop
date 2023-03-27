#include <windows.h>
#include<string.h>
#include<stdio.h>
#include<stdlib.h>
/*screen methods*/
int Screen_get_width(){

    int screenwidth_real = GetSystemMetrics(SM_CXSCREEN);
    return screenwidth_real;

}

int Screen_get_height(){

    int screenheight_real = GetSystemMetrics(SM_CYSCREEN);
    return screenheight_real;

}



/*mouse methods*/
int Mouse_get_pos_x(){

    POINT p ;
    GetCursorPos(&p);
    return p.x;

}

int Mouse_get_pos_y(){

    POINT p ;
    GetCursorPos(&p);
    return p.y;

}


int Mouse_move_to(int x ,int y ){

    int screen_width = Screen_get_width();
    int screen_height = Screen_get_height();


    if((x<0) || (x>screen_width)){

        return 0;

    }
    if((y<0) ||(y>screen_height)){
        return 0;
    }

    long scr_x = 65535/screen_width;
    long scr_y = 65535/screen_height;
    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dx = scr_x*x;
    inputs[0].mi.dy  = scr_y*y;
    inputs[0].mi.dwFlags = MOUSEEVENTF_MOVE| MOUSEEVENTF_ABSOLUTE ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));
    return 1;
    
}

void Mouse_left_press(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_LEFTDOWN ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Mouse_left_release(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_LEFTUP ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Mouse_right_press(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_RIGHTDOWN ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Mouse_right_release(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_RIGHTUP ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Mouse_middle_press(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Mouse_middle_release(){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_MIDDLEUP ;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));
    
}

int Mouse_middle_scroll(int s){
    if ((s>INT_MAX) || (s<INT_MIN)){

        return 0;
    }

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_MOUSE;
    inputs[0].mi.dwFlags = MOUSEEVENTF_WHEEL ;
    inputs[0].mi.mouseData = s;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

    return 1;

}

int Mouse_lock_window(wchar_t* window_name,  LPRECT rect){
    
    HWND h = FindWindowW(NULL,window_name);
    int r = GetWindowRect(h,rect);
    return r;

}
    
/*keyboard methods*/
void Key_press(int key){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_KEYBOARD;
    inputs[0].ki.wVk = key ;
    inputs[0].ki.dwFlags = 0;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Key_release(int key){

    INPUT inputs[1] ;
    ZeroMemory(inputs, sizeof(inputs));
    inputs[0].type = INPUT_KEYBOARD;
    inputs[0].ki.wVk = key ;
    inputs[0].ki.dwFlags = KEYEVENTF_KEYUP;
    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));

}

void Key_text(const wchar_t* s,unsigned char a[3]){
     
	// size_t len = strlen(t) + 1; 
	// size_t converted = 0; 
    // wchar_t *s; 
	// s=(wchar_t*)malloc(len*sizeof(wchar_t)); 
	// mbstowcs_s(&converted, s, len, t, _TRUNCATE);
	 
    
    OpenClipboard(NULL);
    EmptyClipboard();
    HANDLE hData = GlobalAlloc(GMEM_MOVEABLE, (wcslen(s)+1)*sizeof(wchar_t));
    LPWSTR pData = (LPWSTR)GlobalLock(hData);
    CopyMemory(pData, s,  (wcslen(s))*sizeof(wchar_t));
    GlobalUnlock(hData);
    SetClipboardData(CF_UNICODETEXT, hData);
    CloseClipboard();
   
    Key_press(a[0]);
    Key_press(a[1]);
    Key_press(a[2]);
    Key_release(a[0]);
    Key_release(a[1]);
    Key_release(a[2]);
    

    

}


