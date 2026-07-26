<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>main_Qun l ti khon h thng</name>
   <tag></tag>
   <elementGuidId>95ace36c-dc7f-44ea-b0d6-492d8af14fc1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>.main</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' main ')]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Quản lý tài khoản hệ thống Tất cả vai trò Quản trị viên Dược sĩ Khách hàng Tất c&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>main</value>
      <webElementGuid>94a91ca2-061d-4426-b76d-fb989ca3dca9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>main</value>
      <webElementGuid>5b5385be-b8a1-40a2-aa17-f5448bca6742</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        
            
                
            
            Quản lý tài khoản hệ thống
        
        
            

    
        
        
    
        
            
            
        
        
            Tất cả vai trò
            Quản trị viên
            Dược sĩ
            Khách hàng
        
        
            Tất cả trạng thái
            Đang hoạt động
            Đã khóa
        
        Đặt lại
    



    
        
            
                
                    Mã người dùng
                    Họ và tên
                    Email liên hệ
                    Số điện thoại
                    Vai trò
                    Trạng thái
                    Thao tác xử lý
                
            
            
                
                    USR-000011
                    
                        
                            VN
                            Dược sĩ Hoàng Văn Nam 
                        
                    
                    hoangnam.ds@gmail.com
                    0915678901
                    Dược sĩ
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000008
                    
                        
                            TH
                            Dược sĩ Lê Thanh Hoa 
                        
                    
                    thanhhoa.ds@gmail.com
                    0912345678
                    Dược sĩ
                    Đã khóa
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000006
                    
                        
                            VE
                            Hoàng Văn E 
                        
                    
                    hoangvane@gmail.com
                    0905678901
                    Khách hàng
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
        
    
    
        
        Không tìm thấy tài khoản người dùng nào
    




    
        
            Chi tiết thông tin tài khoản
            ×
        
        
            
        
        
            Đóng cửa sổ
        
    




    
        
            Phân quyền chức năng tài khoản
            ×
        
        
            
                
                
                    
                        Họ tên người dùng
                        
                    
                    
                        Chọn cấu hình vai trò mới *
                        
                            Khách hàng (CUSTOMER)
                            Dược sĩ (PHARMACIST)
                            Quản trị viên (ADMIN)
                        
                    
                
            
        
        
            Hủy bỏ
            Xác nhận cấp quyền
        
    



    
    Thao tác thành công



    let searchTimeout;
    let toastTimer;

    const LOGGED_IN_ADMIN_ID = 1;

    function openModal(id) {
        document.getElementById(id).classList.remove('hidden');
    }

    function closeModal(id) {
        document.getElementById(id).classList.add('hidden');
    }

    document.querySelectorAll('[data-close]').forEach(btn => {
        btn.addEventListener('click', () => closeModal(btn.dataset.close));
    });

    function showLocalToast(msg) {
        const toast = document.getElementById('localToast');
        document.getElementById('localToastMsg').textContent = msg;
        toast.classList.add('show');
        clearTimeout(toastTimer);
        toastTimer = setTimeout(() => toast.classList.remove('show'), 3000);
    }

    function getInitials(name) {
        return name.split(' ').map(w => w[0]).slice(-2).join('').toUpperCase();
    }

    function fetchUserList() {
        const search = document.getElementById('searchInput').value.trim();
        const vaiTro = document.getElementById('filterRole').value;
        const trangThai = document.getElementById('filterStatus').value;
        const url = `http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/getList?search=${encodeURIComponent(search)}&amp;vaiTro=${vaiTro}&amp;trangThai=${trangThai}`;

        fetch(url)
            .then(res => res.json())
            .then(res => {
                if (res.status) renderTable(res.data);
            })
            .catch(err => console.error(&quot;Lỗi lấy danh sách tài khoản:&quot;, err));
    }

    function renderTable(list) {
        const tbody = document.getElementById('tableBody');
        const emptyState = document.getElementById('emptyState');

        if (list.length === 0) {
            tbody.innerHTML = '';
            emptyState.style.display = 'block';
            return;
        }

        emptyState.style.display = 'none';
        tbody.innerHTML = list.map(user => {
            const roleClass = user.vaiTro === 'QUAN_TRI_VIEN' ? 'badge-role-admin' : (user.vaiTro === 'DUOC_SI' ? 'badge-role-pharmacist' : 'badge-role-customer');
            const roleLabel = user.vaiTro === 'QUAN_TRI_VIEN' ? 'Quản trị viên' : (user.vaiTro === 'DUOC_SI' ? 'Dược sĩ' : 'Khách hàng');
            const statusClass = user.trangThai ? 'badge-status-active' : 'badge-status-locked';
            const statusLabel = user.trangThai ? 'Hoạt động' : 'Đã khóa';


            const lockIcon = user.trangThai ? `&lt;i class=&quot;fa-solid fa-lock&quot;>&lt;/i>` : `&lt;i class=&quot;fa-solid fa-lock-open&quot;>&lt;/i>`;
            const isSelf = user.idNguoiDung == LOGGED_IN_ADMIN_ID;
            const isAdminRow = user.vaiTro === 'QUAN_TRI_VIEN';

            const disabledAttr = (isSelf || isAdminRow) ? 'disabled title=&quot;Bạn không được phép tự xử lý chính mình hoặc thao tác lên tài khoản quản trị viên khác!&quot;' : '';

            return `
                &lt;tr class=&quot;${user.trangThai ? '' : 'row-inactive'}&quot;>
                    &lt;td class=&quot;cell-mono cell-strong&quot;>USR-${String(user.idNguoiDung).padStart(6, '0')}&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;user-cell&quot;>
                            &lt;div class=&quot;user-avatar&quot;>${getInitials(user.hoTen)}&lt;/div>
                            &lt;div class=&quot;cell-strong&quot;>${user.hoTen} ${isSelf ? '&lt;small style=&quot;color:var(--green-700); font-weight:700;&quot;>(Bạn)&lt;/small>' : ''}&lt;/div>
                        &lt;/div>
                    &lt;/td>
                    &lt;td>${user.email}&lt;/td>
                    &lt;td class=&quot;cell-mono&quot;>${user.soDienThoai || '—'}&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${roleClass}&quot;>${roleLabel}&lt;/span>&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${statusClass}&quot;>${statusLabel}&lt;/span>&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;actions-cell&quot;>
                            &lt;button class=&quot;action-btn view&quot; onclick=&quot;openDetailModal(${user.idNguoiDung})&quot; title=&quot;Xem hồ sơ chi tiết&quot;>
                                &lt;i class=&quot;fa-solid fa-eye&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn edit&quot; onclick=&quot;openRoleModal(${user.idNguoiDung}, '${user.hoTen}', '${user.vaiTro}')&quot; ${disabledAttr}>
                                &lt;i class=&quot;fa-solid fa-sliders&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn lock&quot; onclick=&quot;toggleAccountStatus(${user.idNguoiDung}, '${user.hoTen}')&quot; ${disabledAttr}>
                                ${lockIcon}
                            &lt;/button>
                        &lt;/div>
                    &lt;/td>
                &lt;/tr>
            `;
        }).join('');
    }

    function openDetailModal(id) {
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/detail/${id}`)
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    const u = res.data;
                    let extHTML = '';

                    if (u.vaiTro === 'KHACH_HANG') {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Điểm tích lũy&lt;/div>&lt;div class=&quot;v&quot; style=&quot;color:var(--green-700);&quot;>${u.diemTichLuy || 0} điểm&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Ngày sinh&lt;/div>&lt;div class=&quot;v&quot;>${u.ngaySinh || '—'}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Địa chỉ giao hàng mặc định&lt;/div>&lt;div class=&quot;v&quot;>${u.diaChiGiaoHang || '—'}&lt;/div>&lt;/div>
                        `;
                    } else if (u.vaiTro === 'DUOC_SI') {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số chứng chỉ hành nghề&lt;/div>&lt;div class=&quot;v&quot;>${u.chungChiHanhNghe || '—'}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trình độ chuyên môn&lt;/div>&lt;div class=&quot;v&quot;>${u.trinhDo || '—'}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Nơi cấp bằng / chứng chỉ&lt;/div>&lt;div class=&quot;v&quot;>${u.noiCap || '—'}&lt;/div>&lt;/div>
                        `;
                    }

                    document.getElementById('detailBody').innerHTML = `
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Mã số tài khoản&lt;/div>&lt;div class=&quot;v cell-mono&quot;>USR-${String(u.idNguoiDung).padStart(6, '0')}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Họ và tên&lt;/div>&lt;div class=&quot;v&quot;>${u.hoTen}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Địa chỉ Email&lt;/div>&lt;div class=&quot;v&quot;>${u.email}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số điện thoại&lt;/div>&lt;div class=&quot;v cell-mono&quot;>${u.soDienThoai || '—'}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Phân quyền hệ thống&lt;/div>&lt;div class=&quot;v&quot;>&lt;b style=&quot;color:var(--blue-600);&quot;>${u.vaiTro}&lt;/b>&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trạng thái đăng nhập&lt;/div>&lt;div class=&quot;v&quot;>${u.trangThai ? 'Đang hoạt động' : 'Đang bị khóa'}&lt;/div>&lt;/div>
                        ${extHTML}
                    `;
                    openModal('modalDetail');
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lấy chi tiết người dùng:&quot;, err));
    }

    function openRoleModal(id, name, currentRole) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Hệ thống chặn: Bạn không thể tự thay đổi vai trò của chính mình!&quot;);
            return;
        }
        document.getElementById('f_role_id').value = id;
        document.getElementById('f_role_name').value = name;
        document.getElementById('f_role_select').value = currentRole;
        openModal('modalRole');
    }

    document.getElementById('btnSaveRole').addEventListener('click', () => {
        const formData = new FormData(document.getElementById('roleForm'));
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/saveRole`, {
                method: 'POST',
                body: formData
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeModal('modalRole');
                    showLocalToast(res.message);
                    fetchUserList();
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lưu quyền hạn tài khoản:&quot;, err));
    });

    function toggleAccountStatus(id, name) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Quy tắc an toàn: Bạn không được phép tự khóa chính tài khoản Admin của mình!&quot;);
            return;
        }
        if (confirm(`Xác nhận chuyển đổi trạng thái hoạt động (Khóa/Mở khóa) của tài khoản &quot;${name}&quot;?`)) {
            fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/toggleStatus/${id}`, {
                    method: 'POST'
                })
                .then(res => res.json())
                .then(res => {
                    if (res.status) {
                        showLocalToast(res.message);
                        fetchUserList();
                    } else {
                        alert(res.message);
                    }
                })
                .catch(err => console.error(&quot;Lỗi cập nhật trạng thái tài khoản:&quot;, err));
        }
    }

    document.getElementById('searchInput').addEventListener('input', () => {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(fetchUserList, 350);
    });
    document.getElementById('filterRole').addEventListener('change', fetchUserList);
    document.getElementById('filterStatus').addEventListener('change', fetchUserList);

    document.getElementById('btnResetFilter').addEventListener('click', () => {
        document.getElementById('searchInput').value = '';
        document.getElementById('filterRole').value = 'all';
        document.getElementById('filterStatus').value = 'all';
        fetchUserList();
    });

    fetchUserList();
    
</value>
      <webElementGuid>0f84f319-0094-42f9-a70a-f49a1081896b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-9797d51057b9c28239c83e66dbf229c4</value>
      <webElementGuid>319465e8-a4dd-4a8f-aaaa-6dba67d34946</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' main ')]</value>
      <webElementGuid>dff0a55b-71bd-44d9-97bc-6f91dc43fb5c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' main ')]</value>
      <webElementGuid>583ebcf7-727f-4194-851a-4153c9c9c9a7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//main[(text() = concat(&quot;
    
        
            
                
            
            Quản lý tài khoản hệ thống
        
        
            

    
        
        
    
        
            
            
        
        
            Tất cả vai trò
            Quản trị viên
            Dược sĩ
            Khách hàng
        
        
            Tất cả trạng thái
            Đang hoạt động
            Đã khóa
        
        Đặt lại
    



    
        
            
                
                    Mã người dùng
                    Họ và tên
                    Email liên hệ
                    Số điện thoại
                    Vai trò
                    Trạng thái
                    Thao tác xử lý
                
            
            
                
                    USR-000011
                    
                        
                            VN
                            Dược sĩ Hoàng Văn Nam 
                        
                    
                    hoangnam.ds@gmail.com
                    0915678901
                    Dược sĩ
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000008
                    
                        
                            TH
                            Dược sĩ Lê Thanh Hoa 
                        
                    
                    thanhhoa.ds@gmail.com
                    0912345678
                    Dược sĩ
                    Đã khóa
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000006
                    
                        
                            VE
                            Hoàng Văn E 
                        
                    
                    hoangvane@gmail.com
                    0905678901
                    Khách hàng
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
        
    
    
        
        Không tìm thấy tài khoản người dùng nào
    




    
        
            Chi tiết thông tin tài khoản
            ×
        
        
            
        
        
            Đóng cửa sổ
        
    




    
        
            Phân quyền chức năng tài khoản
            ×
        
        
            
                
                
                    
                        Họ tên người dùng
                        
                    
                    
                        Chọn cấu hình vai trò mới *
                        
                            Khách hàng (CUSTOMER)
                            Dược sĩ (PHARMACIST)
                            Quản trị viên (ADMIN)
                        
                    
                
            
        
        
            Hủy bỏ
            Xác nhận cấp quyền
        
    



    
    Thao tác thành công



    let searchTimeout;
    let toastTimer;

    const LOGGED_IN_ADMIN_ID = 1;

    function openModal(id) {
        document.getElementById(id).classList.remove(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    }

    function closeModal(id) {
        document.getElementById(id).classList.add(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    }

    document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-close]&quot; , &quot;'&quot; , &quot;).forEach(btn => {
        btn.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => closeModal(btn.dataset.close));
    });

    function showLocalToast(msg) {
        const toast = document.getElementById(&quot; , &quot;'&quot; , &quot;localToast&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot; , &quot;'&quot; , &quot;localToastMsg&quot; , &quot;'&quot; , &quot;).textContent = msg;
        toast.classList.add(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
        clearTimeout(toastTimer);
        toastTimer = setTimeout(() => toast.classList.remove(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;), 3000);
    }

    function getInitials(name) {
        return name.split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).map(w => w[0]).slice(-2).join(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).toUpperCase();
    }

    function fetchUserList() {
        const search = document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).value.trim();
        const vaiTro = document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).value;
        const trangThai = document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).value;
        const url = `http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/getList?search=${encodeURIComponent(search)}&amp;vaiTro=${vaiTro}&amp;trangThai=${trangThai}`;

        fetch(url)
            .then(res => res.json())
            .then(res => {
                if (res.status) renderTable(res.data);
            })
            .catch(err => console.error(&quot;Lỗi lấy danh sách tài khoản:&quot;, err));
    }

    function renderTable(list) {
        const tbody = document.getElementById(&quot; , &quot;'&quot; , &quot;tableBody&quot; , &quot;'&quot; , &quot;);
        const emptyState = document.getElementById(&quot; , &quot;'&quot; , &quot;emptyState&quot; , &quot;'&quot; , &quot;);

        if (list.length === 0) {
            tbody.innerHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            emptyState.style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
            return;
        }

        emptyState.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
        tbody.innerHTML = list.map(user => {
            const roleClass = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;badge-role-admin&quot; , &quot;'&quot; , &quot; : (user.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;badge-role-pharmacist&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;badge-role-customer&quot; , &quot;'&quot; , &quot;);
            const roleLabel = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Quản trị viên&quot; , &quot;'&quot; , &quot; : (user.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Dược sĩ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Khách hàng&quot; , &quot;'&quot; , &quot;);
            const statusClass = user.trangThai ? &quot; , &quot;'&quot; , &quot;badge-status-active&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;badge-status-locked&quot; , &quot;'&quot; , &quot;;
            const statusLabel = user.trangThai ? &quot; , &quot;'&quot; , &quot;Hoạt động&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Đã khóa&quot; , &quot;'&quot; , &quot;;


            const lockIcon = user.trangThai ? `&lt;i class=&quot;fa-solid fa-lock&quot;>&lt;/i>` : `&lt;i class=&quot;fa-solid fa-lock-open&quot;>&lt;/i>`;
            const isSelf = user.idNguoiDung == LOGGED_IN_ADMIN_ID;
            const isAdminRow = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot;;

            const disabledAttr = (isSelf || isAdminRow) ? &quot; , &quot;'&quot; , &quot;disabled title=&quot;Bạn không được phép tự xử lý chính mình hoặc thao tác lên tài khoản quản trị viên khác!&quot;&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

            return `
                &lt;tr class=&quot;${user.trangThai ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;row-inactive&quot; , &quot;'&quot; , &quot;}&quot;>
                    &lt;td class=&quot;cell-mono cell-strong&quot;>USR-${String(user.idNguoiDung).padStart(6, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)}&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;user-cell&quot;>
                            &lt;div class=&quot;user-avatar&quot;>${getInitials(user.hoTen)}&lt;/div>
                            &lt;div class=&quot;cell-strong&quot;>${user.hoTen} ${isSelf ? &quot; , &quot;'&quot; , &quot;&lt;small style=&quot;color:var(--green-700); font-weight:700;&quot;>(Bạn)&lt;/small>&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}&lt;/div>
                        &lt;/div>
                    &lt;/td>
                    &lt;td>${user.email}&lt;/td>
                    &lt;td class=&quot;cell-mono&quot;>${user.soDienThoai || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${roleClass}&quot;>${roleLabel}&lt;/span>&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${statusClass}&quot;>${statusLabel}&lt;/span>&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;actions-cell&quot;>
                            &lt;button class=&quot;action-btn view&quot; onclick=&quot;openDetailModal(${user.idNguoiDung})&quot; title=&quot;Xem hồ sơ chi tiết&quot;>
                                &lt;i class=&quot;fa-solid fa-eye&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn edit&quot; onclick=&quot;openRoleModal(${user.idNguoiDung}, &quot; , &quot;'&quot; , &quot;${user.hoTen}&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;${user.vaiTro}&quot; , &quot;'&quot; , &quot;)&quot; ${disabledAttr}>
                                &lt;i class=&quot;fa-solid fa-sliders&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn lock&quot; onclick=&quot;toggleAccountStatus(${user.idNguoiDung}, &quot; , &quot;'&quot; , &quot;${user.hoTen}&quot; , &quot;'&quot; , &quot;)&quot; ${disabledAttr}>
                                ${lockIcon}
                            &lt;/button>
                        &lt;/div>
                    &lt;/td>
                &lt;/tr>
            `;
        }).join(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    }

    function openDetailModal(id) {
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/detail/${id}`)
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    const u = res.data;
                    let extHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

                    if (u.vaiTro === &quot; , &quot;'&quot; , &quot;KHACH_HANG&quot; , &quot;'&quot; , &quot;) {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Điểm tích lũy&lt;/div>&lt;div class=&quot;v&quot; style=&quot;color:var(--green-700);&quot;>${u.diemTichLuy || 0} điểm&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Ngày sinh&lt;/div>&lt;div class=&quot;v&quot;>${u.ngaySinh || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Địa chỉ giao hàng mặc định&lt;/div>&lt;div class=&quot;v&quot;>${u.diaChiGiaoHang || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        `;
                    } else if (u.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot;) {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số chứng chỉ hành nghề&lt;/div>&lt;div class=&quot;v&quot;>${u.chungChiHanhNghe || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trình độ chuyên môn&lt;/div>&lt;div class=&quot;v&quot;>${u.trinhDo || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Nơi cấp bằng / chứng chỉ&lt;/div>&lt;div class=&quot;v&quot;>${u.noiCap || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        `;
                    }

                    document.getElementById(&quot; , &quot;'&quot; , &quot;detailBody&quot; , &quot;'&quot; , &quot;).innerHTML = `
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Mã số tài khoản&lt;/div>&lt;div class=&quot;v cell-mono&quot;>USR-${String(u.idNguoiDung).padStart(6, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Họ và tên&lt;/div>&lt;div class=&quot;v&quot;>${u.hoTen}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Địa chỉ Email&lt;/div>&lt;div class=&quot;v&quot;>${u.email}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số điện thoại&lt;/div>&lt;div class=&quot;v cell-mono&quot;>${u.soDienThoai || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Phân quyền hệ thống&lt;/div>&lt;div class=&quot;v&quot;>&lt;b style=&quot;color:var(--blue-600);&quot;>${u.vaiTro}&lt;/b>&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trạng thái đăng nhập&lt;/div>&lt;div class=&quot;v&quot;>${u.trangThai ? &quot; , &quot;'&quot; , &quot;Đang hoạt động&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Đang bị khóa&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        ${extHTML}
                    `;
                    openModal(&quot; , &quot;'&quot; , &quot;modalDetail&quot; , &quot;'&quot; , &quot;);
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lấy chi tiết người dùng:&quot;, err));
    }

    function openRoleModal(id, name, currentRole) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Hệ thống chặn: Bạn không thể tự thay đổi vai trò của chính mình!&quot;);
            return;
        }
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_id&quot; , &quot;'&quot; , &quot;).value = id;
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_name&quot; , &quot;'&quot; , &quot;).value = name;
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_select&quot; , &quot;'&quot; , &quot;).value = currentRole;
        openModal(&quot; , &quot;'&quot; , &quot;modalRole&quot; , &quot;'&quot; , &quot;);
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;btnSaveRole&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => {
        const formData = new FormData(document.getElementById(&quot; , &quot;'&quot; , &quot;roleForm&quot; , &quot;'&quot; , &quot;));
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/saveRole`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                body: formData
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeModal(&quot; , &quot;'&quot; , &quot;modalRole&quot; , &quot;'&quot; , &quot;);
                    showLocalToast(res.message);
                    fetchUserList();
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lưu quyền hạn tài khoản:&quot;, err));
    });

    function toggleAccountStatus(id, name) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Quy tắc an toàn: Bạn không được phép tự khóa chính tài khoản Admin của mình!&quot;);
            return;
        }
        if (confirm(`Xác nhận chuyển đổi trạng thái hoạt động (Khóa/Mở khóa) của tài khoản &quot;${name}&quot;?`)) {
            fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/toggleStatus/${id}`, {
                    method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
                })
                .then(res => res.json())
                .then(res => {
                    if (res.status) {
                        showLocalToast(res.message);
                        fetchUserList();
                    } else {
                        alert(res.message);
                    }
                })
                .catch(err => console.error(&quot;Lỗi cập nhật trạng thái tài khoản:&quot;, err));
        }
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, () => {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(fetchUserList, 350);
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, fetchUserList);
    document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, fetchUserList);

    document.getElementById(&quot; , &quot;'&quot; , &quot;btnResetFilter&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => {
        document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;;
        fetchUserList();
    });

    fetchUserList();
    
&quot;) or . = concat(&quot;
    
        
            
                
            
            Quản lý tài khoản hệ thống
        
        
            

    
        
        
    
        
            
            
        
        
            Tất cả vai trò
            Quản trị viên
            Dược sĩ
            Khách hàng
        
        
            Tất cả trạng thái
            Đang hoạt động
            Đã khóa
        
        Đặt lại
    



    
        
            
                
                    Mã người dùng
                    Họ và tên
                    Email liên hệ
                    Số điện thoại
                    Vai trò
                    Trạng thái
                    Thao tác xử lý
                
            
            
                
                    USR-000011
                    
                        
                            VN
                            Dược sĩ Hoàng Văn Nam 
                        
                    
                    hoangnam.ds@gmail.com
                    0915678901
                    Dược sĩ
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000008
                    
                        
                            TH
                            Dược sĩ Lê Thanh Hoa 
                        
                    
                    thanhhoa.ds@gmail.com
                    0912345678
                    Dược sĩ
                    Đã khóa
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
                
                    USR-000006
                    
                        
                            VE
                            Hoàng Văn E 
                        
                    
                    hoangvane@gmail.com
                    0905678901
                    Khách hàng
                    Hoạt động
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                
            
        
    
    
        
        Không tìm thấy tài khoản người dùng nào
    




    
        
            Chi tiết thông tin tài khoản
            ×
        
        
            
        
        
            Đóng cửa sổ
        
    




    
        
            Phân quyền chức năng tài khoản
            ×
        
        
            
                
                
                    
                        Họ tên người dùng
                        
                    
                    
                        Chọn cấu hình vai trò mới *
                        
                            Khách hàng (CUSTOMER)
                            Dược sĩ (PHARMACIST)
                            Quản trị viên (ADMIN)
                        
                    
                
            
        
        
            Hủy bỏ
            Xác nhận cấp quyền
        
    



    
    Thao tác thành công



    let searchTimeout;
    let toastTimer;

    const LOGGED_IN_ADMIN_ID = 1;

    function openModal(id) {
        document.getElementById(id).classList.remove(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    }

    function closeModal(id) {
        document.getElementById(id).classList.add(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    }

    document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-close]&quot; , &quot;'&quot; , &quot;).forEach(btn => {
        btn.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => closeModal(btn.dataset.close));
    });

    function showLocalToast(msg) {
        const toast = document.getElementById(&quot; , &quot;'&quot; , &quot;localToast&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot; , &quot;'&quot; , &quot;localToastMsg&quot; , &quot;'&quot; , &quot;).textContent = msg;
        toast.classList.add(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
        clearTimeout(toastTimer);
        toastTimer = setTimeout(() => toast.classList.remove(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;), 3000);
    }

    function getInitials(name) {
        return name.split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).map(w => w[0]).slice(-2).join(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).toUpperCase();
    }

    function fetchUserList() {
        const search = document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).value.trim();
        const vaiTro = document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).value;
        const trangThai = document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).value;
        const url = `http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/getList?search=${encodeURIComponent(search)}&amp;vaiTro=${vaiTro}&amp;trangThai=${trangThai}`;

        fetch(url)
            .then(res => res.json())
            .then(res => {
                if (res.status) renderTable(res.data);
            })
            .catch(err => console.error(&quot;Lỗi lấy danh sách tài khoản:&quot;, err));
    }

    function renderTable(list) {
        const tbody = document.getElementById(&quot; , &quot;'&quot; , &quot;tableBody&quot; , &quot;'&quot; , &quot;);
        const emptyState = document.getElementById(&quot; , &quot;'&quot; , &quot;emptyState&quot; , &quot;'&quot; , &quot;);

        if (list.length === 0) {
            tbody.innerHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            emptyState.style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
            return;
        }

        emptyState.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
        tbody.innerHTML = list.map(user => {
            const roleClass = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;badge-role-admin&quot; , &quot;'&quot; , &quot; : (user.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;badge-role-pharmacist&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;badge-role-customer&quot; , &quot;'&quot; , &quot;);
            const roleLabel = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Quản trị viên&quot; , &quot;'&quot; , &quot; : (user.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Dược sĩ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Khách hàng&quot; , &quot;'&quot; , &quot;);
            const statusClass = user.trangThai ? &quot; , &quot;'&quot; , &quot;badge-status-active&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;badge-status-locked&quot; , &quot;'&quot; , &quot;;
            const statusLabel = user.trangThai ? &quot; , &quot;'&quot; , &quot;Hoạt động&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Đã khóa&quot; , &quot;'&quot; , &quot;;


            const lockIcon = user.trangThai ? `&lt;i class=&quot;fa-solid fa-lock&quot;>&lt;/i>` : `&lt;i class=&quot;fa-solid fa-lock-open&quot;>&lt;/i>`;
            const isSelf = user.idNguoiDung == LOGGED_IN_ADMIN_ID;
            const isAdminRow = user.vaiTro === &quot; , &quot;'&quot; , &quot;QUAN_TRI_VIEN&quot; , &quot;'&quot; , &quot;;

            const disabledAttr = (isSelf || isAdminRow) ? &quot; , &quot;'&quot; , &quot;disabled title=&quot;Bạn không được phép tự xử lý chính mình hoặc thao tác lên tài khoản quản trị viên khác!&quot;&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

            return `
                &lt;tr class=&quot;${user.trangThai ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;row-inactive&quot; , &quot;'&quot; , &quot;}&quot;>
                    &lt;td class=&quot;cell-mono cell-strong&quot;>USR-${String(user.idNguoiDung).padStart(6, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)}&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;user-cell&quot;>
                            &lt;div class=&quot;user-avatar&quot;>${getInitials(user.hoTen)}&lt;/div>
                            &lt;div class=&quot;cell-strong&quot;>${user.hoTen} ${isSelf ? &quot; , &quot;'&quot; , &quot;&lt;small style=&quot;color:var(--green-700); font-weight:700;&quot;>(Bạn)&lt;/small>&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}&lt;/div>
                        &lt;/div>
                    &lt;/td>
                    &lt;td>${user.email}&lt;/td>
                    &lt;td class=&quot;cell-mono&quot;>${user.soDienThoai || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${roleClass}&quot;>${roleLabel}&lt;/span>&lt;/td>
                    &lt;td>&lt;span class=&quot;badge ${statusClass}&quot;>${statusLabel}&lt;/span>&lt;/td>
                    &lt;td>
                        &lt;div class=&quot;actions-cell&quot;>
                            &lt;button class=&quot;action-btn view&quot; onclick=&quot;openDetailModal(${user.idNguoiDung})&quot; title=&quot;Xem hồ sơ chi tiết&quot;>
                                &lt;i class=&quot;fa-solid fa-eye&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn edit&quot; onclick=&quot;openRoleModal(${user.idNguoiDung}, &quot; , &quot;'&quot; , &quot;${user.hoTen}&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;${user.vaiTro}&quot; , &quot;'&quot; , &quot;)&quot; ${disabledAttr}>
                                &lt;i class=&quot;fa-solid fa-sliders&quot;>&lt;/i>
                            &lt;/button>
                            &lt;button class=&quot;action-btn lock&quot; onclick=&quot;toggleAccountStatus(${user.idNguoiDung}, &quot; , &quot;'&quot; , &quot;${user.hoTen}&quot; , &quot;'&quot; , &quot;)&quot; ${disabledAttr}>
                                ${lockIcon}
                            &lt;/button>
                        &lt;/div>
                    &lt;/td>
                &lt;/tr>
            `;
        }).join(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    }

    function openDetailModal(id) {
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/detail/${id}`)
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    const u = res.data;
                    let extHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

                    if (u.vaiTro === &quot; , &quot;'&quot; , &quot;KHACH_HANG&quot; , &quot;'&quot; , &quot;) {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Điểm tích lũy&lt;/div>&lt;div class=&quot;v&quot; style=&quot;color:var(--green-700);&quot;>${u.diemTichLuy || 0} điểm&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Ngày sinh&lt;/div>&lt;div class=&quot;v&quot;>${u.ngaySinh || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Địa chỉ giao hàng mặc định&lt;/div>&lt;div class=&quot;v&quot;>${u.diaChiGiaoHang || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        `;
                    } else if (u.vaiTro === &quot; , &quot;'&quot; , &quot;DUOC_SI&quot; , &quot;'&quot; , &quot;) {
                        extHTML = `
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số chứng chỉ hành nghề&lt;/div>&lt;div class=&quot;v&quot;>${u.chungChiHanhNghe || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trình độ chuyên môn&lt;/div>&lt;div class=&quot;v&quot;>${u.trinhDo || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                            &lt;div class=&quot;detail-item span-2&quot;>&lt;div class=&quot;k&quot;>Nơi cấp bằng / chứng chỉ&lt;/div>&lt;div class=&quot;v&quot;>${u.noiCap || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        `;
                    }

                    document.getElementById(&quot; , &quot;'&quot; , &quot;detailBody&quot; , &quot;'&quot; , &quot;).innerHTML = `
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Mã số tài khoản&lt;/div>&lt;div class=&quot;v cell-mono&quot;>USR-${String(u.idNguoiDung).padStart(6, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Họ và tên&lt;/div>&lt;div class=&quot;v&quot;>${u.hoTen}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Địa chỉ Email&lt;/div>&lt;div class=&quot;v&quot;>${u.email}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Số điện thoại&lt;/div>&lt;div class=&quot;v cell-mono&quot;>${u.soDienThoai || &quot; , &quot;'&quot; , &quot;—&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Phân quyền hệ thống&lt;/div>&lt;div class=&quot;v&quot;>&lt;b style=&quot;color:var(--blue-600);&quot;>${u.vaiTro}&lt;/b>&lt;/div>&lt;/div>
                        &lt;div class=&quot;detail-item&quot;>&lt;div class=&quot;k&quot;>Trạng thái đăng nhập&lt;/div>&lt;div class=&quot;v&quot;>${u.trangThai ? &quot; , &quot;'&quot; , &quot;Đang hoạt động&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Đang bị khóa&quot; , &quot;'&quot; , &quot;}&lt;/div>&lt;/div>
                        ${extHTML}
                    `;
                    openModal(&quot; , &quot;'&quot; , &quot;modalDetail&quot; , &quot;'&quot; , &quot;);
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lấy chi tiết người dùng:&quot;, err));
    }

    function openRoleModal(id, name, currentRole) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Hệ thống chặn: Bạn không thể tự thay đổi vai trò của chính mình!&quot;);
            return;
        }
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_id&quot; , &quot;'&quot; , &quot;).value = id;
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_name&quot; , &quot;'&quot; , &quot;).value = name;
        document.getElementById(&quot; , &quot;'&quot; , &quot;f_role_select&quot; , &quot;'&quot; , &quot;).value = currentRole;
        openModal(&quot; , &quot;'&quot; , &quot;modalRole&quot; , &quot;'&quot; , &quot;);
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;btnSaveRole&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => {
        const formData = new FormData(document.getElementById(&quot; , &quot;'&quot; , &quot;roleForm&quot; , &quot;'&quot; , &quot;));
        fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/saveRole`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                body: formData
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeModal(&quot; , &quot;'&quot; , &quot;modalRole&quot; , &quot;'&quot; , &quot;);
                    showLocalToast(res.message);
                    fetchUserList();
                } else {
                    alert(res.message);
                }
            })
            .catch(err => console.error(&quot;Lỗi lưu quyền hạn tài khoản:&quot;, err));
    });

    function toggleAccountStatus(id, name) {
        if (id == LOGGED_IN_ADMIN_ID) {
            alert(&quot;Quy tắc an toàn: Bạn không được phép tự khóa chính tài khoản Admin của mình!&quot;);
            return;
        }
        if (confirm(`Xác nhận chuyển đổi trạng thái hoạt động (Khóa/Mở khóa) của tài khoản &quot;${name}&quot;?`)) {
            fetch(`http://localhost/BanThuoc/public/admin/quanLyTaiKhoan/toggleStatus/${id}`, {
                    method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
                })
                .then(res => res.json())
                .then(res => {
                    if (res.status) {
                        showLocalToast(res.message);
                        fetchUserList();
                    } else {
                        alert(res.message);
                    }
                })
                .catch(err => console.error(&quot;Lỗi cập nhật trạng thái tài khoản:&quot;, err));
        }
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, () => {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(fetchUserList, 350);
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, fetchUserList);
    document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, fetchUserList);

    document.getElementById(&quot; , &quot;'&quot; , &quot;btnResetFilter&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => {
        document.getElementById(&quot; , &quot;'&quot; , &quot;searchInput&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot; , &quot;'&quot; , &quot;filterRole&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot; , &quot;'&quot; , &quot;filterStatus&quot; , &quot;'&quot; , &quot;).value = &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;;
        fetchUserList();
    });

    fetchUserList();
    
&quot;))]</value>
      <webElementGuid>86a99210-36e2-443f-a08d-0c14a876e8c0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
