#[doc = "Register `PHYSET` reader"]
pub struct R(crate::R<PHYSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYSET` writer"]
pub struct W(crate::W<PHYSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYSET_SPEC>;
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
impl From<crate::W<PHYSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRPD` reader - Power-Down Control"]
pub type DIRPD_R = crate::BitReader<DIRPD_A>;
#[doc = "Power-Down Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRPD_A {
    #[doc = "0: Does not enter low-power consumption mode"]
    _0 = 0,
    #[doc = "1: Enter low-power consumption mode"]
    _1 = 1,
}
impl From<DIRPD_A> for bool {
    #[inline(always)]
    fn from(variant: DIRPD_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRPD_A {
        match self.bits {
            false => DIRPD_A::_0,
            true => DIRPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRPD_A::_1
    }
}
#[doc = "Field `DIRPD` writer - Power-Down Control"]
pub type DIRPD_W<'a, const O: u8> = crate::BitWriter<'a, u16, PHYSET_SPEC, DIRPD_A, O>;
impl<'a, const O: u8> DIRPD_W<'a, O> {
    #[doc = "Does not enter low-power consumption mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRPD_A::_0)
    }
    #[doc = "Enter low-power consumption mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRPD_A::_1)
    }
}
#[doc = "Field `PLLRESET` reader - PLL Reset Control"]
pub type PLLRESET_R = crate::BitReader<PLLRESET_A>;
#[doc = "PLL Reset Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRESET_A {
    #[doc = "0: Disable PLL reset control for UTMI_PHY"]
    _0 = 0,
    #[doc = "1: Enable PLL reset control for UTMI_PHY"]
    _1 = 1,
}
impl From<PLLRESET_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRESET_A {
        match self.bits {
            false => PLLRESET_A::_0,
            true => PLLRESET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLRESET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLRESET_A::_1
    }
}
#[doc = "Field `PLLRESET` writer - PLL Reset Control"]
pub type PLLRESET_W<'a, const O: u8> = crate::BitWriter<'a, u16, PHYSET_SPEC, PLLRESET_A, O>;
impl<'a, const O: u8> PLLRESET_W<'a, O> {
    #[doc = "Disable PLL reset control for UTMI_PHY"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLRESET_A::_0)
    }
    #[doc = "Enable PLL reset control for UTMI_PHY"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLRESET_A::_1)
    }
}
#[doc = "Field `CDPEN` reader - Charging Downstream Port Enable"]
pub type CDPEN_R = crate::BitReader<CDPEN_A>;
#[doc = "Charging Downstream Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDPEN_A {
    #[doc = "0: Disable charging downstream port"]
    _0 = 0,
    #[doc = "1: Enable charging downstream port"]
    _1 = 1,
}
impl From<CDPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CDPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDPEN_A {
        match self.bits {
            false => CDPEN_A::_0,
            true => CDPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDPEN_A::_1
    }
}
#[doc = "Field `CDPEN` writer - Charging Downstream Port Enable"]
pub type CDPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PHYSET_SPEC, CDPEN_A, O>;
impl<'a, const O: u8> CDPEN_W<'a, O> {
    #[doc = "Disable charging downstream port"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDPEN_A::_0)
    }
    #[doc = "Enable charging downstream port"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDPEN_A::_1)
    }
}
#[doc = "Field `CLKSEL` reader - Input System Clock Frequency"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Input System Clock Frequency\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Setting Prohibited"]
    _00 = 0,
    #[doc = "1: 12 MHz"]
    _01 = 1,
    #[doc = "2: 20 MHz"]
    _10 = 2,
    #[doc = "3: 24 MHz"]
    _11 = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::_00,
            1 => CLKSEL_A::_01,
            2 => CLKSEL_A::_10,
            3 => CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKSEL_A::_11
    }
}
#[doc = "Field `CLKSEL` writer - Input System Clock Frequency"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PHYSET_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKSEL_A::_00)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKSEL_A::_01)
    }
    #[doc = "20 MHz"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKSEL_A::_10)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKSEL_A::_11)
    }
}
#[doc = "Field `REPSEL` reader - Terminating Resistance Adjustment Cycle"]
pub type REPSEL_R = crate::FieldReader<u8, REPSEL_A>;
#[doc = "Terminating Resistance Adjustment Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPSEL_A {
    #[doc = "0: No cycle is set."]
    _00 = 0,
    #[doc = "1: Adjust terminating resistance at 16-second intervals."]
    _01 = 1,
    #[doc = "2: Adjust terminating resistance at 64-second intervals."]
    _10 = 2,
    #[doc = "3: Adjust terminating resistance at 128-second intervals."]
    _11 = 3,
}
impl From<REPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REPSEL_A) -> Self {
        variant as _
    }
}
impl REPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPSEL_A {
        match self.bits {
            0 => REPSEL_A::_00,
            1 => REPSEL_A::_01,
            2 => REPSEL_A::_10,
            3 => REPSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == REPSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == REPSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == REPSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == REPSEL_A::_11
    }
}
#[doc = "Field `REPSEL` writer - Terminating Resistance Adjustment Cycle"]
pub type REPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PHYSET_SPEC, u8, REPSEL_A, 2, O>;
impl<'a, const O: u8> REPSEL_W<'a, O> {
    #[doc = "No cycle is set."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(REPSEL_A::_00)
    }
    #[doc = "Adjust terminating resistance at 16-second intervals."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(REPSEL_A::_01)
    }
    #[doc = "Adjust terminating resistance at 64-second intervals."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(REPSEL_A::_10)
    }
    #[doc = "Adjust terminating resistance at 128-second intervals."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(REPSEL_A::_11)
    }
}
#[doc = "Field `REPSTART` reader - Forcibly Start Terminating Resistance Adjustment"]
pub type REPSTART_R = crate::BitReader<REPSTART_A>;
#[doc = "Forcibly Start Terminating Resistance Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPSTART_A {
    #[doc = "0: Terminating resistance adjustment is forcibly started"]
    _0 = 0,
    #[doc = "1: Terminating resistance adjustment is not forcibly started"]
    _1 = 1,
}
impl From<REPSTART_A> for bool {
    #[inline(always)]
    fn from(variant: REPSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl REPSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPSTART_A {
        match self.bits {
            false => REPSTART_A::_0,
            true => REPSTART_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REPSTART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REPSTART_A::_1
    }
}
#[doc = "Field `REPSTART` writer - Forcibly Start Terminating Resistance Adjustment"]
pub type REPSTART_W<'a, const O: u8> = crate::BitWriter<'a, u16, PHYSET_SPEC, REPSTART_A, O>;
impl<'a, const O: u8> REPSTART_W<'a, O> {
    #[doc = "Terminating resistance adjustment is forcibly started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REPSTART_A::_0)
    }
    #[doc = "Terminating resistance adjustment is not forcibly started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REPSTART_A::_1)
    }
}
#[doc = "Field `HSEB` reader - CL-Only Mode"]
pub type HSEB_R = crate::BitReader<HSEB_A>;
#[doc = "CL-Only Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEB_A {
    #[doc = "0: CL-only mode is not activated."]
    _0 = 0,
    #[doc = "1: CL-only mode is activated."]
    _1 = 1,
}
impl From<HSEB_A> for bool {
    #[inline(always)]
    fn from(variant: HSEB_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEB_A {
        match self.bits {
            false => HSEB_A::_0,
            true => HSEB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSEB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSEB_A::_1
    }
}
#[doc = "Field `HSEB` writer - CL-Only Mode"]
pub type HSEB_W<'a, const O: u8> = crate::BitWriter<'a, u16, PHYSET_SPEC, HSEB_A, O>;
impl<'a, const O: u8> HSEB_W<'a, O> {
    #[doc = "CL-only mode is not activated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSEB_A::_0)
    }
    #[doc = "CL-only mode is activated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSEB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Power-Down Control"]
    #[inline(always)]
    pub fn dirpd(&self) -> DIRPD_R {
        DIRPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Reset Control"]
    #[inline(always)]
    pub fn pllreset(&self) -> PLLRESET_R {
        PLLRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Charging Downstream Port Enable"]
    #[inline(always)]
    pub fn cdpen(&self) -> CDPEN_R {
        CDPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Input System Clock Frequency"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Terminating Resistance Adjustment Cycle"]
    #[inline(always)]
    pub fn repsel(&self) -> REPSEL_R {
        REPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Forcibly Start Terminating Resistance Adjustment"]
    #[inline(always)]
    pub fn repstart(&self) -> REPSTART_R {
        REPSTART_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - CL-Only Mode"]
    #[inline(always)]
    pub fn hseb(&self) -> HSEB_R {
        HSEB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-Down Control"]
    #[inline(always)]
    #[must_use]
    pub fn dirpd(&mut self) -> DIRPD_W<0> {
        DIRPD_W::new(self)
    }
    #[doc = "Bit 1 - PLL Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn pllreset(&mut self) -> PLLRESET_W<1> {
        PLLRESET_W::new(self)
    }
    #[doc = "Bit 3 - Charging Downstream Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdpen(&mut self) -> CDPEN_W<3> {
        CDPEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Input System Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<4> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Terminating Resistance Adjustment Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn repsel(&mut self) -> REPSEL_W<8> {
        REPSEL_W::new(self)
    }
    #[doc = "Bit 11 - Forcibly Start Terminating Resistance Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn repstart(&mut self) -> REPSTART_W<11> {
        REPSTART_W::new(self)
    }
    #[doc = "Bit 15 - CL-Only Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hseb(&mut self) -> HSEB_W<15> {
        HSEB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physet](index.html) module"]
pub struct PHYSET_SPEC;
impl crate::RegisterSpec for PHYSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [physet::R](R) reader structure"]
impl crate::Readable for PHYSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [physet::W](W) writer structure"]
impl crate::Writable for PHYSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHYSET to value 0x33"]
impl crate::Resettable for PHYSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x33;
}
