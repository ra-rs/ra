#[doc = "Register `MB%s_ID` reader"]
pub struct R(crate::R<MB_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MB_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MB_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MB_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MB%s_ID` writer"]
pub struct W(crate::W<MB_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MB_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MB_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MB_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EID` reader - Extended ID"]
pub type EID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EID` writer - Extended ID"]
pub type EID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MB_ID_SPEC, u32, u32, 18, O>;
#[doc = "Field `SID` reader - Standard ID"]
pub type SID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SID` writer - Standard ID"]
pub type SID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MB_ID_SPEC, u16, u16, 11, O>;
#[doc = "Field `RTR` reader - Remote Transmission Request"]
pub type RTR_R = crate::BitReader<RTR_A>;
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR_A {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::_0,
            true => RTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTR_A::_1
    }
}
#[doc = "Field `RTR` writer - Remote Transmission Request"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MB_ID_SPEC, RTR_A, O>;
impl<'a, const O: u8> RTR_W<'a, O> {
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTR_A::_0)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTR_A::_1)
    }
}
#[doc = "Field `IDE` reader - ID Extension"]
pub type IDE_R = crate::BitReader<IDE_A>;
#[doc = "ID Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE_A {
    #[doc = "0: Standard ID"]
    _0 = 0,
    #[doc = "1: Extended ID"]
    _1 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::_0,
            true => IDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDE_A::_1
    }
}
#[doc = "Field `IDE` writer - ID Extension"]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MB_ID_SPEC, IDE_A, O>;
impl<'a, const O: u8> IDE_W<'a, O> {
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDE_A::_0)
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Remote Transmission Request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ID Extension"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    #[must_use]
    pub fn eid(&mut self) -> EID_W<0> {
        EID_W::new(self)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SID_W<18> {
        SID_W::new(self)
    }
    #[doc = "Bit 30 - Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<30> {
        RTR_W::new(self)
    }
    #[doc = "Bit 31 - ID Extension"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<31> {
        IDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mb_id](index.html) module"]
pub struct MB_ID_SPEC;
impl crate::RegisterSpec for MB_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mb_id::R](R) reader structure"]
impl crate::Readable for MB_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mb_id::W](W) writer structure"]
impl crate::Writable for MB_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_ID to value 0"]
impl crate::Resettable for MB_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
