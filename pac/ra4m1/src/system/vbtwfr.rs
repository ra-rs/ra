#[doc = "Register `VBTWFR` reader"]
pub struct R(crate::R<VBTWFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTWFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTWFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTWFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTWFR` writer"]
pub struct W(crate::W<VBTWFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTWFR_SPEC>;
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
impl From<crate::W<VBTWFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTWFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCH0F` reader - VBATWIO0 Wakeup Trigger Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VCH0F_R = crate::BitReader<VCH0F_A>;
#[doc = "VBATWIO0 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0F_A {
    #[doc = "0: No wakeup trigger by the VBATWIO0 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO0 pin is generated"]
    _1 = 1,
}
impl From<VCH0F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0F_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH0F_A {
        match self.bits {
            false => VCH0F_A::_0,
            true => VCH0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0F_A::_1
    }
}
#[doc = "Field `VCH0F` writer - VBATWIO0 Wakeup Trigger Flag"]
pub type VCH0F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTWFR_SPEC, VCH0F_A, O>;
impl<'a, const O: u8> VCH0F_W<'a, O> {
    #[doc = "No wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH0F_A::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH0F_A::_1)
    }
}
#[doc = "Field `VCH1F` reader - VBATWIO1 Wakeup Trigger Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VCH1F_R = crate::BitReader<VCH1F_A>;
#[doc = "VBATWIO1 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1F_A {
    #[doc = "0: No wakeup trigger by the VBATWIO1 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO1 pin is generated"]
    _1 = 1,
}
impl From<VCH1F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1F_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH1F_A {
        match self.bits {
            false => VCH1F_A::_0,
            true => VCH1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1F_A::_1
    }
}
#[doc = "Field `VCH1F` writer - VBATWIO1 Wakeup Trigger Flag"]
pub type VCH1F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTWFR_SPEC, VCH1F_A, O>;
impl<'a, const O: u8> VCH1F_W<'a, O> {
    #[doc = "No wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH1F_A::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH1F_A::_1)
    }
}
#[doc = "Field `VCH2F` reader - VBATWIO2 Wakeup Trigger Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VCH2F_R = crate::BitReader<VCH2F_A>;
#[doc = "VBATWIO2 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2F_A {
    #[doc = "0: No wakeup trigger by the VBATWIO2 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO2 pin is generated"]
    _1 = 1,
}
impl From<VCH2F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2F_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH2F_A {
        match self.bits {
            false => VCH2F_A::_0,
            true => VCH2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2F_A::_1
    }
}
#[doc = "Field `VCH2F` writer - VBATWIO2 Wakeup Trigger Flag"]
pub type VCH2F_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTWFR_SPEC, VCH2F_A, O>;
impl<'a, const O: u8> VCH2F_W<'a, O> {
    #[doc = "No wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH2F_A::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH2F_A::_1)
    }
}
#[doc = "Field `VRTCIF` reader - VBATT RTC-Interval Wakeup Trigger Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VRTCIF_R = crate::BitReader<VRTCIF_A>;
#[doc = "VBATT RTC-Interval Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCIF_A {
    #[doc = "0: No wakeup trigger by the RTC interval is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the RTC interval is generated"]
    _1 = 1,
}
impl From<VRTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCIF_A) -> Self {
        variant as u8 != 0
    }
}
impl VRTCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRTCIF_A {
        match self.bits {
            false => VRTCIF_A::_0,
            true => VRTCIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCIF_A::_1
    }
}
#[doc = "Field `VRTCIF` writer - VBATT RTC-Interval Wakeup Trigger Flag"]
pub type VRTCIF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTWFR_SPEC, VRTCIF_A, O>;
impl<'a, const O: u8> VRTCIF_W<'a, O> {
    #[doc = "No wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRTCIF_A::_0)
    }
    #[doc = "A wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRTCIF_A::_1)
    }
}
#[doc = "Field `VRTCAF` reader - VBATT RTC-Alarm Wakeup Trigger Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VRTCAF_R = crate::BitReader<VRTCAF_A>;
#[doc = "VBATT RTC-Alarm Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCAF_A {
    #[doc = "0: No wakeup trigger by the RTC alarm is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the RTC alarm is generated"]
    _1 = 1,
}
impl From<VRTCAF_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCAF_A) -> Self {
        variant as u8 != 0
    }
}
impl VRTCAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRTCAF_A {
        match self.bits {
            false => VRTCAF_A::_0,
            true => VRTCAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCAF_A::_1
    }
}
#[doc = "Field `VRTCAF` writer - VBATT RTC-Alarm Wakeup Trigger Flag"]
pub type VRTCAF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTWFR_SPEC, VRTCAF_A, O>;
impl<'a, const O: u8> VRTCAF_W<'a, O> {
    #[doc = "No wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRTCAF_A::_0)
    }
    #[doc = "A wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRTCAF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch0f(&self) -> VCH0F_R {
        VCH0F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch1f(&self) -> VCH1F_R {
        VCH1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch2f(&self) -> VCH2F_R {
        VCH2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcif(&self) -> VRTCIF_R {
        VRTCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcaf(&self) -> VRTCAF_R {
        VRTCAF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vch0f(&mut self) -> VCH0F_W<0> {
        VCH0F_W::new(self)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vch1f(&mut self) -> VCH1F_W<1> {
        VCH1F_W::new(self)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vch2f(&mut self) -> VCH2F_W<2> {
        VCH2F_W::new(self)
    }
    #[doc = "Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrtcif(&mut self) -> VRTCIF_W<3> {
        VRTCIF_W::new(self)
    }
    #[doc = "Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrtcaf(&mut self) -> VRTCAF_W<4> {
        VRTCAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Wakeup trigger source Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtwfr](index.html) module"]
pub struct VBTWFR_SPEC;
impl crate::RegisterSpec for VBTWFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtwfr::R](R) reader structure"]
impl crate::Readable for VBTWFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtwfr::W](W) writer structure"]
impl crate::Writable for VBTWFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWFR to value 0"]
impl crate::Resettable for VBTWFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
