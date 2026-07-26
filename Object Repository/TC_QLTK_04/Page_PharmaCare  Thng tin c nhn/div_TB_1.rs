<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_TB_1</name>
   <tag></tag>
   <elementGuidId>33ee408e-b070-41ca-a2e0-7afb28cadc2d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' page ')]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>.page</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>div >> internal:has-text=&quot;TB Trần Thị B Tài khoản đã xác thực Hồ sơ hiện tại Họ và tên Số điện thoại (Tên &quot;i >> nth=0</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>b24cfe3b-f64d-4ef9-873f-f1be81573bc5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page</value>
      <webElementGuid>bce8bd46-e477-4080-8a22-3b6ad620ffd3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    


    
        
            TB
            
                Trần Thị B
                
                    
                    Tài khoản đã xác thực
                
                
            
        

        Hồ sơ hiện tại

        
            
                Họ và tên
                
            
            
                Số điện thoại (Tên đăng nhập)
                
                    
                    
                
            
            
                Email chính
                
            
        
        
            
                
                Địa chỉ thường trú
                
            
        

        
            
                Sửa thông tin
             Lưu thay đổi
            Hủy
        
    

    
        
            Địa chỉ giao hàng
             Thêm địa
                chỉ
        

        
        
                                                
                        
                            
                        
                        
                            
                                
                                Địa chỉ giao hàng
                                Mặc định                            
                            
                                Trần Thị B
                                
                                0902345678
                            
                            456 Đường Mậu Thân, Phường Xuân Khánh, Quận Ninh Kiều, Cần Thơ
                        
                        
                            Sửa
                            Đặt mặc định
                            Xoá
                        
                    
                                    
    





    
        
            Thêm địa chỉ giao hàng
            
                
            
        
        
            
                
                    
                        
                        Nhãn tên địa chỉ *
                        
                    
                

                
                    
                        Tên người nhận *
                        
                    
                    
                        Số điện thoại *
                        
                    
                

                
                
                    
                        Địa chỉ giao hàng đầy đủ *
                        
                        VD: 12 Trần Hưng Đạo, Phường 1, TP. Vĩnh Long
                    
                

                
                    
                        
                        Ghi chú giao hàng
                        
                    
                

                
                    
                    Đặt làm địa chỉ mặc định ngay khi tạo
                
            
        
        
            Hủy
             Lưu địa chỉ
        
    



    const editableIds = ['hoVaTen', 'emailChinh', 'diaChi'];
    // Lưu ý: &quot;diaChi&quot; (Địa chỉ thường trú) vẫn cho sửa trên giao diện như bản gốc,
    // nhưng KHÔNG được gửi lên server / lưu CSDL vì chưa có cột tương ứng.

    function toggleEdit(editing) {
        editableIds.forEach(id => {
            document.getElementById(id).disabled = !editing;
        });
        document.getElementById('btnEdit').disabled = editing;
        document.getElementById('btnSave').disabled = !editing;
        document.getElementById('btnCancel').disabled = !editing;
    }

    function getInitials(name) {
        const parts = name.split(' ').filter(Boolean);
        if (parts.length === 0) return '';
        const last = parts[parts.length - 1][0] || '';
        const first = parts[0][0] || '';
        return (first + last).toUpperCase();
    }

    // Lưu Họ tên + Email thật xuống CSDL (bảng NguoiDung)
    function saveInfo() {
        const fullName = document.getElementById('hoVaTen').value.trim();
        const email = document.getElementById('emailChinh').value.trim();

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/capNhatThongTin`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                },
                body: `hoTen=${encodeURIComponent(fullName)}&amp;email=${encodeURIComponent(email)}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    document.getElementById('displayName').textContent = fullName;
                    document.getElementById('avatarInitials').textContent = getInitials(fullName);
                    toggleEdit(false);
                } else {
                    alert(res.message || 'Cập nhật thất bại, vui lòng thử lại.');
                }
            })
            .catch(() => alert('Lỗi kết nối máy chủ.'));
    }

    // ══ MODAL ĐỊA CHỈ ══
    const addrModalOverlay = document.getElementById('addrModalOverlay');

    function openAddressModal() {
        document.getElementById('addrForm').reset();
        document.getElementById('mRecipient').value = document.getElementById('hoVaTen').value;
        document.getElementById('mPhone').value = document.getElementById('soDienThoai').value;
        addrModalOverlay.classList.add('open');
        document.body.style.overflow = 'hidden';
    }

    function closeAddressModal() {
        addrModalOverlay.classList.remove('open');
        document.body.style.overflow = '';
    }
    addrModalOverlay.addEventListener('click', (e) => {
        if (e.target === addrModalOverlay) closeAddressModal();
    });

    // Thêm địa chỉ giao hàng thật xuống CSDL (bảng DiaChiGiaoHang)
    // &quot;Nhãn tên địa chỉ&quot; và &quot;Ghi chú giao hàng&quot; vẫn bắt buộc nhập trên form như cũ,
    // nhưng KHÔNG gửi lên server vì bảng chưa có cột lưu 2 trường này.
    function submitAddress() {
        const addrLabel = document.getElementById('mLabel').value.trim();
        const recipient = document.getElementById('mRecipient').value.trim();
        const phone = document.getElementById('mPhone').value.trim();
        const detail = document.getElementById('mDetail').value.trim();
        const isDefault = document.getElementById('mDefault').checked;

        if (!addrLabel || !recipient || !phone || !detail) {
            alert('Vui lòng điền đầy đủ các trường bắt buộc (*)');
            return;
        }

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/themDiaChi`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                },
                body: `tenNguoiNhan=${encodeURIComponent(recipient)}&amp;soDienThoaiNhan=${encodeURIComponent(phone)}&amp;diaChiChiTiet=${encodeURIComponent(detail)}&amp;laMacDinh=${isDefault ? 1 : 0}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeAddressModal();
                    window.location.reload();
                } else {
                    alert(res.message || 'Thêm địa chỉ thất bại, vui lòng thử lại.');
                }
            })
            .catch(() => alert('Lỗi kết nối máy chủ.'));
    }

    // Xoá địa chỉ (bảng DiaChiGiaoHang)
    document.getElementById('addressList').addEventListener('click', function(e) {
        const deleteBtn = e.target.closest('.delete-link');
        if (!deleteBtn) return;
        const item = deleteBtn.closest('.addr-item');
        const idDiaChi = item.dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/xoaDiaChi/${idDiaChi}`, {
                method: 'POST'
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    item.remove();
                } else {
                    alert(res.message || 'Xoá địa chỉ thất bại, vui lòng thử lại.');
                }
            })
            .catch(() => alert('Lỗi kết nối máy chủ.'));
    });

    // Đặt địa chỉ mặc định (bảng DiaChiGiaoHang)
    document.getElementById('addressList').addEventListener('click', function(e) {
        const defaultBtn = e.target.closest('.setdefault-link');
        if (!defaultBtn) return;
        const idDiaChi = defaultBtn.closest('.addr-item').dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/datMacDinh/${idDiaChi}`, {
                method: 'POST'
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    window.location.reload();
                } else {
                    alert(res.message || 'Đặt mặc định thất bại, vui lòng thử lại.');
                }
            })
            .catch(() => alert('Lỗi kết nối máy chủ.'));
    });
</value>
      <webElementGuid>0a600111-e230-460d-89a7-54260b2c90ac</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-b9a386d60a6b93aa7c93a82d8c0d1c39</value>
      <webElementGuid>386b5672-53ff-4f84-82f3-797d86334e59</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' page ')]</value>
      <webElementGuid>1c2da79f-bf81-4867-b5c8-b02f171fdd78</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' page ')]</value>
      <webElementGuid>99e7c555-e31c-48d4-b2e9-446b270b5923</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    


    
        
            TB
            
                Trần Thị B
                
                    
                    Tài khoản đã xác thực
                
                
            
        

        Hồ sơ hiện tại

        
            
                Họ và tên
                
            
            
                Số điện thoại (Tên đăng nhập)
                
                    
                    
                
            
            
                Email chính
                
            
        
        
            
                
                Địa chỉ thường trú
                
            
        

        
            
                Sửa thông tin
             Lưu thay đổi
            Hủy
        
    

    
        
            Địa chỉ giao hàng
             Thêm địa
                chỉ
        

        
        
                                                
                        
                            
                        
                        
                            
                                
                                Địa chỉ giao hàng
                                Mặc định                            
                            
                                Trần Thị B
                                
                                0902345678
                            
                            456 Đường Mậu Thân, Phường Xuân Khánh, Quận Ninh Kiều, Cần Thơ
                        
                        
                            Sửa
                            Đặt mặc định
                            Xoá
                        
                    
                                    
    





    
        
            Thêm địa chỉ giao hàng
            
                
            
        
        
            
                
                    
                        
                        Nhãn tên địa chỉ *
                        
                    
                

                
                    
                        Tên người nhận *
                        
                    
                    
                        Số điện thoại *
                        
                    
                

                
                
                    
                        Địa chỉ giao hàng đầy đủ *
                        
                        VD: 12 Trần Hưng Đạo, Phường 1, TP. Vĩnh Long
                    
                

                
                    
                        
                        Ghi chú giao hàng
                        
                    
                

                
                    
                    Đặt làm địa chỉ mặc định ngay khi tạo
                
            
        
        
            Hủy
             Lưu địa chỉ
        
    



    const editableIds = [&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;emailChinh&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;diaChi&quot; , &quot;'&quot; , &quot;];
    // Lưu ý: &quot;diaChi&quot; (Địa chỉ thường trú) vẫn cho sửa trên giao diện như bản gốc,
    // nhưng KHÔNG được gửi lên server / lưu CSDL vì chưa có cột tương ứng.

    function toggleEdit(editing) {
        editableIds.forEach(id => {
            document.getElementById(id).disabled = !editing;
        });
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnEdit&quot; , &quot;'&quot; , &quot;).disabled = editing;
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnSave&quot; , &quot;'&quot; , &quot;).disabled = !editing;
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnCancel&quot; , &quot;'&quot; , &quot;).disabled = !editing;
    }

    function getInitials(name) {
        const parts = name.split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).filter(Boolean);
        if (parts.length === 0) return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const last = parts[parts.length - 1][0] || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const first = parts[0][0] || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        return (first + last).toUpperCase();
    }

    // Lưu Họ tên + Email thật xuống CSDL (bảng NguoiDung)
    function saveInfo() {
        const fullName = document.getElementById(&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;).value.trim();
        const email = document.getElementById(&quot; , &quot;'&quot; , &quot;emailChinh&quot; , &quot;'&quot; , &quot;).value.trim();

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/capNhatThongTin`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                headers: {
                    &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;
                },
                body: `hoTen=${encodeURIComponent(fullName)}&amp;email=${encodeURIComponent(email)}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    document.getElementById(&quot; , &quot;'&quot; , &quot;displayName&quot; , &quot;'&quot; , &quot;).textContent = fullName;
                    document.getElementById(&quot; , &quot;'&quot; , &quot;avatarInitials&quot; , &quot;'&quot; , &quot;).textContent = getInitials(fullName);
                    toggleEdit(false);
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Cập nhật thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    }

    // ══ MODAL ĐỊA CHỈ ══
    const addrModalOverlay = document.getElementById(&quot; , &quot;'&quot; , &quot;addrModalOverlay&quot; , &quot;'&quot; , &quot;);

    function openAddressModal() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;addrForm&quot; , &quot;'&quot; , &quot;).reset();
        document.getElementById(&quot; , &quot;'&quot; , &quot;mRecipient&quot; , &quot;'&quot; , &quot;).value = document.getElementById(&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;).value;
        document.getElementById(&quot; , &quot;'&quot; , &quot;mPhone&quot; , &quot;'&quot; , &quot;).value = document.getElementById(&quot; , &quot;'&quot; , &quot;soDienThoai&quot; , &quot;'&quot; , &quot;).value;
        addrModalOverlay.classList.add(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
        document.body.style.overflow = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
    }

    function closeAddressModal() {
        addrModalOverlay.classList.remove(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
        document.body.style.overflow = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    }
    addrModalOverlay.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, (e) => {
        if (e.target === addrModalOverlay) closeAddressModal();
    });

    // Thêm địa chỉ giao hàng thật xuống CSDL (bảng DiaChiGiaoHang)
    // &quot;Nhãn tên địa chỉ&quot; và &quot;Ghi chú giao hàng&quot; vẫn bắt buộc nhập trên form như cũ,
    // nhưng KHÔNG gửi lên server vì bảng chưa có cột lưu 2 trường này.
    function submitAddress() {
        const addrLabel = document.getElementById(&quot; , &quot;'&quot; , &quot;mLabel&quot; , &quot;'&quot; , &quot;).value.trim();
        const recipient = document.getElementById(&quot; , &quot;'&quot; , &quot;mRecipient&quot; , &quot;'&quot; , &quot;).value.trim();
        const phone = document.getElementById(&quot; , &quot;'&quot; , &quot;mPhone&quot; , &quot;'&quot; , &quot;).value.trim();
        const detail = document.getElementById(&quot; , &quot;'&quot; , &quot;mDetail&quot; , &quot;'&quot; , &quot;).value.trim();
        const isDefault = document.getElementById(&quot; , &quot;'&quot; , &quot;mDefault&quot; , &quot;'&quot; , &quot;).checked;

        if (!addrLabel || !recipient || !phone || !detail) {
            alert(&quot; , &quot;'&quot; , &quot;Vui lòng điền đầy đủ các trường bắt buộc (*)&quot; , &quot;'&quot; , &quot;);
            return;
        }

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/themDiaChi`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                headers: {
                    &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;
                },
                body: `tenNguoiNhan=${encodeURIComponent(recipient)}&amp;soDienThoaiNhan=${encodeURIComponent(phone)}&amp;diaChiChiTiet=${encodeURIComponent(detail)}&amp;laMacDinh=${isDefault ? 1 : 0}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeAddressModal();
                    window.location.reload();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Thêm địa chỉ thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    }

    // Xoá địa chỉ (bảng DiaChiGiaoHang)
    document.getElementById(&quot; , &quot;'&quot; , &quot;addressList&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        const deleteBtn = e.target.closest(&quot; , &quot;'&quot; , &quot;.delete-link&quot; , &quot;'&quot; , &quot;);
        if (!deleteBtn) return;
        const item = deleteBtn.closest(&quot; , &quot;'&quot; , &quot;.addr-item&quot; , &quot;'&quot; , &quot;);
        const idDiaChi = item.dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/xoaDiaChi/${idDiaChi}`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    item.remove();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Xoá địa chỉ thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    });

    // Đặt địa chỉ mặc định (bảng DiaChiGiaoHang)
    document.getElementById(&quot; , &quot;'&quot; , &quot;addressList&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        const defaultBtn = e.target.closest(&quot; , &quot;'&quot; , &quot;.setdefault-link&quot; , &quot;'&quot; , &quot;);
        if (!defaultBtn) return;
        const idDiaChi = defaultBtn.closest(&quot; , &quot;'&quot; , &quot;.addr-item&quot; , &quot;'&quot; , &quot;).dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/datMacDinh/${idDiaChi}`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    window.location.reload();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Đặt mặc định thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    });
&quot;) or . = concat(&quot;
    


    
        
            TB
            
                Trần Thị B
                
                    
                    Tài khoản đã xác thực
                
                
            
        

        Hồ sơ hiện tại

        
            
                Họ và tên
                
            
            
                Số điện thoại (Tên đăng nhập)
                
                    
                    
                
            
            
                Email chính
                
            
        
        
            
                
                Địa chỉ thường trú
                
            
        

        
            
                Sửa thông tin
             Lưu thay đổi
            Hủy
        
    

    
        
            Địa chỉ giao hàng
             Thêm địa
                chỉ
        

        
        
                                                
                        
                            
                        
                        
                            
                                
                                Địa chỉ giao hàng
                                Mặc định                            
                            
                                Trần Thị B
                                
                                0902345678
                            
                            456 Đường Mậu Thân, Phường Xuân Khánh, Quận Ninh Kiều, Cần Thơ
                        
                        
                            Sửa
                            Đặt mặc định
                            Xoá
                        
                    
                                    
    





    
        
            Thêm địa chỉ giao hàng
            
                
            
        
        
            
                
                    
                        
                        Nhãn tên địa chỉ *
                        
                    
                

                
                    
                        Tên người nhận *
                        
                    
                    
                        Số điện thoại *
                        
                    
                

                
                
                    
                        Địa chỉ giao hàng đầy đủ *
                        
                        VD: 12 Trần Hưng Đạo, Phường 1, TP. Vĩnh Long
                    
                

                
                    
                        
                        Ghi chú giao hàng
                        
                    
                

                
                    
                    Đặt làm địa chỉ mặc định ngay khi tạo
                
            
        
        
            Hủy
             Lưu địa chỉ
        
    



    const editableIds = [&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;emailChinh&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;diaChi&quot; , &quot;'&quot; , &quot;];
    // Lưu ý: &quot;diaChi&quot; (Địa chỉ thường trú) vẫn cho sửa trên giao diện như bản gốc,
    // nhưng KHÔNG được gửi lên server / lưu CSDL vì chưa có cột tương ứng.

    function toggleEdit(editing) {
        editableIds.forEach(id => {
            document.getElementById(id).disabled = !editing;
        });
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnEdit&quot; , &quot;'&quot; , &quot;).disabled = editing;
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnSave&quot; , &quot;'&quot; , &quot;).disabled = !editing;
        document.getElementById(&quot; , &quot;'&quot; , &quot;btnCancel&quot; , &quot;'&quot; , &quot;).disabled = !editing;
    }

    function getInitials(name) {
        const parts = name.split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).filter(Boolean);
        if (parts.length === 0) return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const last = parts[parts.length - 1][0] || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const first = parts[0][0] || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        return (first + last).toUpperCase();
    }

    // Lưu Họ tên + Email thật xuống CSDL (bảng NguoiDung)
    function saveInfo() {
        const fullName = document.getElementById(&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;).value.trim();
        const email = document.getElementById(&quot; , &quot;'&quot; , &quot;emailChinh&quot; , &quot;'&quot; , &quot;).value.trim();

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/capNhatThongTin`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                headers: {
                    &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;
                },
                body: `hoTen=${encodeURIComponent(fullName)}&amp;email=${encodeURIComponent(email)}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    document.getElementById(&quot; , &quot;'&quot; , &quot;displayName&quot; , &quot;'&quot; , &quot;).textContent = fullName;
                    document.getElementById(&quot; , &quot;'&quot; , &quot;avatarInitials&quot; , &quot;'&quot; , &quot;).textContent = getInitials(fullName);
                    toggleEdit(false);
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Cập nhật thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    }

    // ══ MODAL ĐỊA CHỈ ══
    const addrModalOverlay = document.getElementById(&quot; , &quot;'&quot; , &quot;addrModalOverlay&quot; , &quot;'&quot; , &quot;);

    function openAddressModal() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;addrForm&quot; , &quot;'&quot; , &quot;).reset();
        document.getElementById(&quot; , &quot;'&quot; , &quot;mRecipient&quot; , &quot;'&quot; , &quot;).value = document.getElementById(&quot; , &quot;'&quot; , &quot;hoVaTen&quot; , &quot;'&quot; , &quot;).value;
        document.getElementById(&quot; , &quot;'&quot; , &quot;mPhone&quot; , &quot;'&quot; , &quot;).value = document.getElementById(&quot; , &quot;'&quot; , &quot;soDienThoai&quot; , &quot;'&quot; , &quot;).value;
        addrModalOverlay.classList.add(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
        document.body.style.overflow = &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
    }

    function closeAddressModal() {
        addrModalOverlay.classList.remove(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
        document.body.style.overflow = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    }
    addrModalOverlay.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, (e) => {
        if (e.target === addrModalOverlay) closeAddressModal();
    });

    // Thêm địa chỉ giao hàng thật xuống CSDL (bảng DiaChiGiaoHang)
    // &quot;Nhãn tên địa chỉ&quot; và &quot;Ghi chú giao hàng&quot; vẫn bắt buộc nhập trên form như cũ,
    // nhưng KHÔNG gửi lên server vì bảng chưa có cột lưu 2 trường này.
    function submitAddress() {
        const addrLabel = document.getElementById(&quot; , &quot;'&quot; , &quot;mLabel&quot; , &quot;'&quot; , &quot;).value.trim();
        const recipient = document.getElementById(&quot; , &quot;'&quot; , &quot;mRecipient&quot; , &quot;'&quot; , &quot;).value.trim();
        const phone = document.getElementById(&quot; , &quot;'&quot; , &quot;mPhone&quot; , &quot;'&quot; , &quot;).value.trim();
        const detail = document.getElementById(&quot; , &quot;'&quot; , &quot;mDetail&quot; , &quot;'&quot; , &quot;).value.trim();
        const isDefault = document.getElementById(&quot; , &quot;'&quot; , &quot;mDefault&quot; , &quot;'&quot; , &quot;).checked;

        if (!addrLabel || !recipient || !phone || !detail) {
            alert(&quot; , &quot;'&quot; , &quot;Vui lòng điền đầy đủ các trường bắt buộc (*)&quot; , &quot;'&quot; , &quot;);
            return;
        }

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/themDiaChi`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                headers: {
                    &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;
                },
                body: `tenNguoiNhan=${encodeURIComponent(recipient)}&amp;soDienThoaiNhan=${encodeURIComponent(phone)}&amp;diaChiChiTiet=${encodeURIComponent(detail)}&amp;laMacDinh=${isDefault ? 1 : 0}`
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    closeAddressModal();
                    window.location.reload();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Thêm địa chỉ thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    }

    // Xoá địa chỉ (bảng DiaChiGiaoHang)
    document.getElementById(&quot; , &quot;'&quot; , &quot;addressList&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        const deleteBtn = e.target.closest(&quot; , &quot;'&quot; , &quot;.delete-link&quot; , &quot;'&quot; , &quot;);
        if (!deleteBtn) return;
        const item = deleteBtn.closest(&quot; , &quot;'&quot; , &quot;.addr-item&quot; , &quot;'&quot; , &quot;);
        const idDiaChi = item.dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/xoaDiaChi/${idDiaChi}`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    item.remove();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Xoá địa chỉ thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    });

    // Đặt địa chỉ mặc định (bảng DiaChiGiaoHang)
    document.getElementById(&quot; , &quot;'&quot; , &quot;addressList&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        const defaultBtn = e.target.closest(&quot; , &quot;'&quot; , &quot;.setdefault-link&quot; , &quot;'&quot; , &quot;);
        if (!defaultBtn) return;
        const idDiaChi = defaultBtn.closest(&quot; , &quot;'&quot; , &quot;.addr-item&quot; , &quot;'&quot; , &quot;).dataset.id;

        fetch(`http://localhost/BanThuoc/public/khachHang/thongTinCaNhan/datMacDinh/${idDiaChi}`, {
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
            })
            .then(res => res.json())
            .then(res => {
                if (res.status) {
                    window.location.reload();
                } else {
                    alert(res.message || &quot; , &quot;'&quot; , &quot;Đặt mặc định thất bại, vui lòng thử lại.&quot; , &quot;'&quot; , &quot;);
                }
            })
            .catch(() => alert(&quot; , &quot;'&quot; , &quot;Lỗi kết nối máy chủ.&quot; , &quot;'&quot; , &quot;));
    });
&quot;))]</value>
      <webElementGuid>bfbfcb7e-0c31-486b-b064-b608ab24a2b9</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
