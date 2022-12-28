#[doc = "Register `MB%s_DL` reader"]
pub struct R(crate::R<MB_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MB_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MB_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MB_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MB%s_DL` writer"]
pub struct W(crate::W<MB_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MB_DL_SPEC>;
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
impl From<crate::W<MB_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MB_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DLC_R = crate::FieldReader<u8, DLC_A>;
#[doc = "Data Length Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLC_A {
    #[doc = "0: Data length = 0 byte"]
    _0000 = 0,
    #[doc = "1: Data length = 1 byte"]
    _0001 = 1,
    #[doc = "2: Data length = 2 bytes"]
    _0010 = 2,
    #[doc = "3: Data length = 3 bytes"]
    _0011 = 3,
    #[doc = "4: Data length = 4 bytes"]
    _0100 = 4,
    #[doc = "5: Data length = 5 bytes"]
    _0101 = 5,
    #[doc = "6: Data length = 6 bytes"]
    _0110 = 6,
    #[doc = "7: Data length = 7 bytes"]
    _0111 = 7,
}
impl From<DLC_A> for u8 {
    #[inline(always)]
    fn from(variant: DLC_A) -> Self {
        variant as _
    }
}
impl DLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DLC_A> {
        match self.bits {
            0 => Some(DLC_A::_0000),
            1 => Some(DLC_A::_0001),
            2 => Some(DLC_A::_0010),
            3 => Some(DLC_A::_0011),
            4 => Some(DLC_A::_0100),
            5 => Some(DLC_A::_0101),
            6 => Some(DLC_A::_0110),
            7 => Some(DLC_A::_0111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DLC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DLC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DLC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DLC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DLC_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DLC_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DLC_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DLC_A::_0111
    }
}
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MB_DL_SPEC, u8, DLC_A, 4, O>;
impl<'a, const O: u8> DLC_W<'a, O> {
    #[doc = "Data length = 0 byte"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DLC_A::_0000)
    }
    #[doc = "Data length = 1 byte"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DLC_A::_0001)
    }
    #[doc = "Data length = 2 bytes"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DLC_A::_0010)
    }
    #[doc = "Data length = 3 bytes"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DLC_A::_0011)
    }
    #[doc = "Data length = 4 bytes"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DLC_A::_0100)
    }
    #[doc = "Data length = 5 bytes"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DLC_A::_0101)
    }
    #[doc = "Data length = 6 bytes"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DLC_A::_0110)
    }
    #[doc = "Data length = 7 bytes"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DLC_A::_0111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mb_dl](index.html) module"]
pub struct MB_DL_SPEC;
impl crate::RegisterSpec for MB_DL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mb_dl::R](R) reader structure"]
impl crate::Readable for MB_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mb_dl::W](W) writer structure"]
impl crate::Writable for MB_DL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_DL to value 0"]
impl crate::Resettable for MB_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
